use portable_atomic::AtomicI8;

pub struct Sequencer {
    pub(crate) current_position: AtomicI8,
    pub(crate) current_tilt: Option<AtomicI8>,
    pub(crate) target_position: AtomicI8,
    pub(crate) target_tilt: Option<AtomicI8>,
}

pub trait ControllerInput {
    fn set_target_position(&self, position: i8);
    fn set_target_tilt(&self, tilt: i8);
    fn interrupt(&self);
}

pub trait ControllerOutput {
    fn next_position(&self) -> i8;
    fn next_tilt(&self) -> i8;
}
