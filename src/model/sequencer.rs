use heapless::Deque;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Direction {
    Extend,
    Retract,
    Hold,
}

#[derive(Debug, Default)]
pub struct HaltingSequencer {
    pub(crate) full_cycle_quantity: u32,
    pub(crate) full_tilt_quantity: Option<u32>,
    pub desired_state: WindowDressingState,
    pub current_state: WindowDressingState,
    pub(crate) instructions: Deque<WindowDressingInstruction, 1024>,
}

pub trait WindowDressingSequencer {
    fn get_next_instruction(&mut self) -> Option<WindowDressingInstruction>;
    fn get_next_instruction_grouped(&mut self) -> Option<WindowDressingInstruction>;
    fn set_state(&mut self, state: WindowDressingState);
    fn set_position(&mut self, position: u8);
    fn set_tilt(&mut self, tilt: i8);
}

pub trait SensingWindowDressingSequencer: WindowDressingSequencer {
    fn trig_endstop(&mut self);
    fn home_fully_opened(&mut self);
    fn home_fully_closed(&mut self);
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct WindowDressingInstruction {
    pub quality: Direction,
    pub quantity: u32,
    pub(crate) completed_state: WindowDressingState,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct WindowDressingState {
    pub position: u8,
    pub tilt: i8,
}
