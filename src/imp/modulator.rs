use crate::{Direction, FixedFrequencyStepperModulator, WindowDressingSequencer};
use core::time::Duration;

impl<S: WindowDressingSequencer> Iterator for FixedFrequencyStepperModulator<S> {
    type Item = (Direction, Duration);

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self
            .cur_instruction
            .as_ref()
            .map(|x| x.quantity)
            .unwrap_or_default();

        if remaining == 0 {
            self.cur_instruction = self.sequencer.get_next_instruction_grouped();
        }

        if let Some(cur) = &mut self.cur_instruction {
            if cur.quantity > 0 {
                cur.quantity -= 1;
                return Some((cur.quality, self.period));
            }
        }
        None
    }
}
