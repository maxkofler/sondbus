use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::transceiver::{State, Transceiver},
    SYNC_SEQUENCE,
};

static SCRATCHPAD: [u8; 0xF] = [0u8; 0xF];

pub fn new_transceiver() -> Transceiver {
    Transceiver::new(&SCRATCHPAD)
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

#[test]
fn test_cmd_nop() {
    let mut t = new_transceiver();
    t.handle(Some(0x55));
    assert_eq!(t.state, State::WaitForCommand);
    t.handle(Some(0x00));
    assert_eq!(t.state, State::WaitForCRC);
    let crc = CRC8Autosar::new().update_move(&[0x55, 0x00]);
    t.handle(Some(crc.finalize()));
    assert_eq!(t.state, State::WaitForStart);
}

#[test]
fn test_cmd_sync() {
    let mut t = new_transceiver();
    t.handle(Some(0x55));
    assert_eq!(t.state, State::WaitForCommand);
    t.handle(Some(0x01));
    assert_eq!(t.state, State::Sync);

    for byte in SYNC_SEQUENCE {
        let res = t.handle(Some(byte));
        assert!(res.is_none());
        assert_eq!(t.state, State::Sync);
    }

    // The protocol version
    let res = t.handle(Some(0x01));
    assert!(res.is_none());
    assert_eq!(t.state, State::WaitForCRC);
    assert!(t.in_sync);

    let crc = CRC8Autosar::new()
        .update_move(&[0x55, 0x01])
        .update_move(&SYNC_SEQUENCE)
        .update_single_move(1);

    let res = t.handle(Some(crc.finalize()));
    assert!(res.is_none());
    assert_eq!(t.state, State::WaitForStart);
}
