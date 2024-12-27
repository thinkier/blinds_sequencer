use crate::WindowDressingSequencer;
use core::time::Duration;

pub struct LinearRampingModulator<S: WindowDressingSequencer> {
    pub(crate) main_period: Duration,
    pub(crate) base_period: Duration,
    pub(crate) ramping_duration: Duration,
    pub(crate) freq_modulation_frequency: u32,
    pub(crate) sequencer: S,
}
