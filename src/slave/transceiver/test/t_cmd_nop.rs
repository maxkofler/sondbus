use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::transceiver::{test::new_transceiver_in_sync, State},
};

#[test]
fn cmd_nop() {
    new_transceiver_in_sync!(t);

    // Send the start byte
    t.handle(Some(0x55));
    assert_eq!(t.state, State::WaitForCommand);

    // The NOP command
    t.handle(Some(0x00));
    assert_eq!(t.state, State::WaitForCRC);

    // Make sure that the transceiver does not respond to a NOP command
    let crc = CRC8Autosar::new().update_move(&[0x55, 0x00]);
    assert!(t.handle(Some(crc.finalize())).is_none());
    assert_eq!(t.state, State::WaitForStart);

    // Make sure SYNC is retained
    assert!(t.in_sync);
}
