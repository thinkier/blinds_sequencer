#[test]
fn sequencer_mem_usage_check() {
    let real = size_of::<crate::model::sequencer::HaltingSequencer>();
    // < 10KiB
    assert!(real < 10 << 10);
}
