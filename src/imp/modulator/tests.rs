use crate::{Direction, FixedFrequencyStepperModulator, HaltingSequencer, WindowDressingSequencer};
use core::time::Duration;

#[test]
fn trivial_modulation() {
    let mut seq = HaltingSequencer::new_roller(100_000);
    seq.current_state.position = 0;
    seq.set_position(100);

    let mut modulator = FixedFrequencyStepperModulator::new(Duration::from_millis(1), seq);

    for _ in 0..100_000 {
        assert_eq!(
            modulator.next(),
            Some((Direction::Retract, Duration::from_millis(1)))
        );
    }
    for _ in 0..500 {
        assert_eq!(
            modulator.next(),
            Some((Direction::Hold, Duration::from_millis(1)))
        );
    }

    assert_eq!(modulator.next(), None);
}
