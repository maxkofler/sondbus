use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::transceiver::{
        test::{new_transceiver_in_sync, test_rx_no_response},
        State,
    },
    CMD_SYNC, PROTOCOL_VERSION_1, START_BYTE, SYNC_SEQUENCE,
};

use super::test_state;

#[test]
fn gain_sync() {
    new_transceiver_in_sync!(t);
    t.in_sync = false;

    test_rx_no_response!(t, START_BYTE);
    test_state!(t, State::WaitForCommand);
    test_rx_no_response!(t, CMD_SYNC);
    test_state!(t, State::Sync);

    for byte in SYNC_SEQUENCE {
        test_rx_no_response!(t, byte);
        assert_eq!(t.state, State::Sync);
    }

    // The protocol version
    test_rx_no_response!(t, PROTOCOL_VERSION_1);
    assert_eq!(t.state, State::WaitForCRC);

    let crc = CRC8Autosar::new()
        .update_move(&[START_BYTE, CMD_SYNC])
        .update_move(&SYNC_SEQUENCE)
        .update_single_move(PROTOCOL_VERSION_1);

    test_rx_no_response!(t, crc.finalize());

    assert!(t.in_sync, "Sync is not gained after correct sync sequence");
    assert_eq!(t.state, State::WaitForStart);
}
