use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::transceiver::{
        state::State,
        test::{new_transceiver_in_sync, test_rx_no_response, test_state, test_sync},
    },
    START_BYTE,
};

#[test]
fn write_o1_s1_complete() {
    let cmd_byte = 0 | 1 << 0; // Operation: Write

    let mut data = vec![START_BYTE, cmd_byte, 0, 1];
    let header_crc = CRC8Autosar::new().update_move(&data);
    data.push(header_crc.finalize());
    data.push(0xAA);
    data.push(CRC8Autosar::new().update_move(&data).finalize());

    new_transceiver_in_sync!(t);
    for b in data {
        test_rx_no_response!(t, b);
    }
    test_state!(t, State::WaitForStart);
    test_sync!(t, true);
}
