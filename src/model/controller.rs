use portable_atomic::AtomicI8;

pub struct Controller {
    position: Option<AtomicI8>,
    tilt: Option<AtomicI8>,
}

pub trait ControllerInput {
    fn set_position(&self, position: i8);
    fn set_tilt(&self, tilt: i8);
    fn interrupt(&self);
}

pub trait ControllerOutput {
    fn drain_position(&self) -> i8;
    fn drain_tilt(&self) -> i8;
}
