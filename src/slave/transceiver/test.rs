use crate::slave::transceiver::{State, TransceiverContext};

static SCRATCHPAD: [u8; 0xF] = [0u8; 0xF];

pub fn new_transceiver() -> TransceiverContext {
    TransceiverContext::new(&SCRATCHPAD)
}

#[test]
fn test_new() {
    let t = new_transceiver();
    assert_eq!(t.state, State::WaitForStart)
}

#[test]
fn test_start() {
    let mut t = new_transceiver();
    let res = t.handle(Some(0x55));
    assert!(res.is_none());
    assert_eq!(t.state, State::WaitForCommand);
}
