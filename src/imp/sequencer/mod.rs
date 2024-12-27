use crate::{Direction, WindowDressingInstruction, WindowDressingSequencer, WindowDressingState};
use core::cmp::Ordering;
use core::ops::AddAssign;

#[cfg(test)]
mod tests;

const HOLD_QUANTITY: u32 = 500;

impl WindowDressingSequencer {
    pub fn new_roller(full_cycle_quantity: u32) -> Self {
        Self {
            full_cycle_quantity,
            ..Default::default()
        }
    }

    pub fn new_venetian(full_cycle_quantity: u32, full_tilt_quantity: u32) -> Self {
        Self {
            full_cycle_quantity,
            full_tilt_quantity: Some(full_tilt_quantity),
            ..Default::default()
        }
    }

    /// Retrieve the next instruction to send to the hardware, if present.
    pub fn get_next_instruction(&mut self) -> Option<WindowDressingInstruction> {
        if let Some(next) = self.instructions.pop_front() {
            self.current_state = next.completed_state;

            // If the instructions queue is empty & it's not commanded to hold, buffer a hold command
            if self.instructions.is_empty() && next.quality != Direction::Hold {
                self.instructions
                    .push_back(WindowDressingInstruction {
                        quality: Direction::Hold,
                        quantity: HOLD_QUANTITY,
                        completed_state: self.current_state,
                    })
                    .expect("The buffer should've been emptied if the hold is queued at the end");
            }

            Some(next)
        } else {
            None
        }
    }

    /// Command from HAP to set the position of the window dressing.
    pub fn set_position(&mut self, opened: u8) {
        self.desired_state.position = opened;
        let tail = self.instructions.pop_back();
        self.instructions.clear();
        let absolute_change = (opened as i8 - self.current_state.position as i8).abs();
        if absolute_change == 0 {
            return;
        }

        let opening = opened > self.current_state.position;
        let quality = if opening {
            Direction::Retract
        } else {
            Direction::Extend
        };

        // Program a pause to prevent directly ramming the system in reverse
        if let Some(tail) = tail {
            if tail.quality != quality {
                self.instructions
                    .push_back(WindowDressingInstruction {
                        quality: Direction::Hold,
                        quantity: HOLD_QUANTITY,
                        completed_state: self.current_state,
                    })
                    .expect("The buffer should be emptied immediately after a set_position");
            }
        }

        let mut angle_while_moving = if opening { -90 } else { 90 };

        self.add_tilt(self.current_state.tilt, angle_while_moving);

        for percentage_change in 1..=absolute_change {
            if self.full_tilt_quantity.is_none() {
                angle_while_moving = 0;
            }

            let mut relative_change = percentage_change as i8;
            if !opening {
                relative_change *= -1;
            }

            let position = (self.current_state.position as i8 + relative_change) as u8;
            // It's safe to eat the error because the state will not be corrupted
            let _ = self.instructions.push_back(WindowDressingInstruction {
                quality,
                quantity: self.full_cycle_quantity / 100,
                completed_state: WindowDressingState {
                    position,
                    tilt: angle_while_moving,
                },
            });
        }
        self.add_tilt(angle_while_moving, self.current_state.tilt);
    }

    /// Command from HAP to set the tilt of the window dressing.
    pub fn set_tilt(&mut self, angle: i8) {
        self.add_tilt(self.get_tail_state().tilt, angle);
    }

    /// Get the desired state of the window dressing, as defined by the last command.
    fn get_tail_state(&self) -> WindowDressingState {
        self.instructions
            .back()
            .map_or(self.current_state, |i| i.completed_state)
    }

    /// Schedules the command necessary to tilt the window dressing.
    fn add_tilt(&mut self, from_angle: i8, to_angle: i8) {
        let opening = to_angle < from_angle;
        let absolute_change = (to_angle as i16 - from_angle as i16).abs();
        if absolute_change == 0 {
            return;
        }
        let tail = self.instructions.back();
        let position = self.get_tail_state().position;

        if let Some(ref full_tilt_quality) = self.full_tilt_quantity {
            self.desired_state.tilt = to_angle;
            let quality = if opening {
                Direction::Retract
            } else {
                Direction::Extend
            };

            if position == 100 {
                // It's safe to eat the error because the state will not be corrupted
                let _ = self.instructions.push_back(WindowDressingInstruction {
                    quality: Direction::Hold,
                    quantity: 0,
                    completed_state: WindowDressingState {
                        position,
                        tilt: to_angle,
                    },
                });
                return;
            }

            if let Some(tail) = tail {
                if tail.quality != quality {
                    // It's safe to eat the error because the state will not be corrupted
                    let _ = self.instructions.push_back(WindowDressingInstruction {
                        quality: Direction::Hold,
                        quantity: HOLD_QUANTITY,
                        completed_state: tail.completed_state,
                    });
                }
            }

            for angle_change in 1..=absolute_change {
                let tilt = if opening {
                    from_angle as i16 - angle_change
                } else {
                    from_angle as i16 + angle_change
                } as i8;

                // It's safe to eat the error because the state will not be corrupted
                let _ = self.instructions.push_back(WindowDressingInstruction {
                    quality,
                    quantity: full_tilt_quality / 180,
                    completed_state: WindowDressingState { position, tilt },
                });
            }
        }
    }

    /// Feedback from hardware that the endstop has been triggered.
    pub fn trig_endstop(&mut self) {
        self.instructions.clear();

        let opening = if self.current_state.position == self.desired_state.position {
            self.current_state.tilt > self.desired_state.tilt
        } else {
            self.current_state.position < self.desired_state.position
                || self.current_state.position == 100
        };
        let tilt = if self.full_tilt_quantity.is_some() {
            if opening {
                0
            } else {
                90
            }
        } else {
            0
        };
        let end_state = WindowDressingState {
            position: if opening { 100 } else { 0 },
            tilt,
        };

        self.current_state = end_state;
        self.desired_state = end_state;
        let _ = self
            .instructions
            .push_back(WindowDressingInstruction {
                quality: Direction::Hold,
                quantity: HOLD_QUANTITY,
                completed_state: end_state,
            })
            .expect("Endstop should've cleared the instructions queue");
    }
}

impl PartialOrd for WindowDressingState {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WindowDressingState {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.position == other.position {
            other.tilt.cmp(&&self.tilt)
        } else {
            self.position.cmp(&other.position)
        }
    }
}

impl AddAssign<&WindowDressingInstruction> for WindowDressingInstruction {
    fn add_assign(&mut self, rhs: &WindowDressingInstruction) {
        if self.quality != rhs.quality {
            panic!("Cannot add instructions with different pulse widths");
        }

        self.quantity += rhs.quantity;
        self.completed_state = rhs.completed_state;
    }
}
