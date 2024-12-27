#[test]
fn sequencer_mem_usage_check() {
    let real = size_of::<crate::model::sequencer::WindowDressingSequencer>();
    // < 10KiB
    assert!(real < 10 << 10);
}
