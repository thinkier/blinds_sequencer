use alloc::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Extend,
    Retract,
    Hold,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowDressingInstruction {
    pub quality: Direction,
    pub quantity: u32,
    pub completed_state: WindowDressingState,
}

#[derive(Debug, Default, PartialEq)]
pub struct WindowDressingSequencer {
    pub full_cycle_quality: u32,
    pub full_tilt_quality: Option<u32>,
    pub desired_state: WindowDressingState,
    pub current_state: WindowDressingState,
    pub instructions: VecDeque<WindowDressingInstruction>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowDressingState {
    pub position: u8,
    pub tilt: i8,
}
