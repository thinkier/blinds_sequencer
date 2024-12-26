use crate::{Sequencer, ControllerInput};
use core::sync::atomic::Ordering;
use portable_atomic::AtomicI8;

impl Sequencer {
    pub fn with_tilt() -> Self {
        Sequencer {
            current_position: AtomicI8::new(0),
            current_tilt: Some(AtomicI8::new(0)),
            target_position: AtomicI8::new(0),
            target_tilt: Some(AtomicI8::new(0)),
        }
    }

    pub fn without_tilt() -> Self {
        Sequencer {
            current_position: AtomicI8::new(0),
            current_tilt: None,
            target_position: AtomicI8::new(0),
            target_tilt: None,
        }
    }
}

impl ControllerInput for Sequencer {
    fn set_target_position(&self, position: i8) {
        self.target_position.store(position, Ordering::Relaxed);
    }

    fn set_target_tilt(&self, tilt: i8) {
        self.target_tilt.as_ref().map(|t| t.store(tilt, Ordering::Relaxed));
    }

    fn interrupt(&self) {
        self.current_position.store(0, Ordering::Relaxed);
        self.current_tilt.as_ref().map(|t| t.store(0, Ordering::Relaxed));
        self.target_position.store(0, Ordering::Relaxed);
        self.target_tilt.as_ref().map(|t| t.store(0, Ordering::Relaxed));
    }
}
