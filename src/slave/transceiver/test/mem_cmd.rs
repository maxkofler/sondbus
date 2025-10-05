use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::transceiver::test::new_transceiver,
    START_BYTE,
};

#[test]
fn test_broadcast() {
    let mut t = new_transceiver();

    t.test_rx_no_response(START_BYTE);
    let cmd_byte = 0 |
        1 << 5 | // Perform a memory command
        1 << 1; // Perform a write

    let mut data = [
        cmd_byte, // The command byte
        0x00,     // We write at offset 0
        0x00,     // We write 0 bytes
        0x00,     // The header CRC (to be set later)
        0x00,     // The CRC (to be set later)
    ];

    t.test_rx_multi_no_response(&data);

    data[3] = CRC8Autosar::new().update_move(&data[0..3]).finalize();
    data[4] = CRC8Autosar::new().update_move(&data[0..4]).finalize();

    t.test_rx_multi_no_response(&data);
}
