use crate::model::sequencer::{
    WindowDressingInstruction, HaltingSequencer, WindowDressingState,
};
use crate::{Direction, WindowDressingSequencer};

#[test]
fn close_grouped() {
    let mut seq = HaltingSequencer::new_roller(100_000);
    seq.current_state.position = 100;
    seq.set_position(0);

    assert_eq!(
        seq.get_next_instruction_grouped(),
        Some(WindowDressingInstruction {
            quality: Direction::Extend,
            quantity: 100_000,
            completed_state: WindowDressingState {
                position: 0,
                tilt: 0
            },
        })
    );

    assert_eq!(
        seq.get_next_instruction_grouped(),
        Some(WindowDressingInstruction {
            quality: Direction::Hold,
            quantity: 500,
            completed_state: WindowDressingState {
                position: 0,
                tilt: 0
            }
        })
    );
}
