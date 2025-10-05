use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::transceiver::{
        state::State,
        test::{
            new_transceiver_in_sync, test_consequence, test_rx_no_response, test_state, test_sync,
            test_tx,
        },
        Consequence,
    },
    START_BYTE,
};

macro_rules! test_mac_write {
    ($t: ident, $data: expr,$consequence:expr,  $addr: expr) => {
        let crc = CRC8Autosar::new().update_move(&$data).finalize();
        new_transceiver_in_sync!($t, $addr);
        for b in $data {
            test_rx_no_response!($t, b);
        }

        test_consequence!($t, $consequence);
        test_rx_no_response!($t, crc);
        test_state!($t, State::WaitForStart);
        test_sync!($t, true);
    };
}

macro_rules! test_logical_write {
    ($t: ident, $data: expr,$consequence:expr, $addr: expr) => {
        let crc = CRC8Autosar::new().update_move(&$data).finalize();
        new_transceiver_in_sync!($t);
        $t.logical_address = $addr;
        for b in $data {
            test_rx_no_response!($t, b);
        }

        test_consequence!($t, $consequence);
        test_rx_no_response!($t, crc);
        test_state!($t, State::WaitForStart);
        test_sync!($t, true);
    };
}

#[test]
#[allow(clippy::identity_op)]
fn write_mac_addressed() {
    let cmd_byte = 1 << 5 // Perform a memory command
        | 1 << 1 // Addressed by MAC
        | 1 << 0; // Operation: Write
    let addr = [1, 2, 3, 4, 5, 6];

    let data = vec![START_BYTE, cmd_byte, 1, 2, 3, 4, 5, 6, 0, 1, 0xAA];
    test_mac_write!(t, data, Consequence::WriteScratchpad, addr);
    assert_eq!(t.mem_cmd_offset, 0, "Processed offset is not right");
    assert_eq!(t.mem_cmd_size, 1, "Processed size is not right");
}

#[test]
#[allow(clippy::identity_op)]
fn write_mac_non_addressed() {
    let cmd_byte = 0
        | 1 << 5 // Perform a memory command
        | 2 << 1 // Addressed by logical address
        | 1 << 0; // Operation: Write
    let addr = [1, 2, 3, 4, 5, 6];

    let data = vec![START_BYTE, cmd_byte, 2, 3, 0, 1, 0xAA];
    test_mac_write!(t, data, Consequence::None, addr);
    assert_eq!(t.mem_cmd_offset, 0, "Processed offset is not right");
    assert_eq!(t.mem_cmd_size, 1, "Processed size is not right");
}

#[test]
#[allow(clippy::identity_op)]
fn write_logical_addressed() {
    let cmd_byte = 0
        | 1 << 5 // Perform a memory command
        | 2 << 1 // Addressed by logical address
        | 1 << 0; // Operation: Write
    let addr = [5, 6];

    let data = vec![START_BYTE, cmd_byte, 5, 6, 0, 1, 0xAA];
    test_logical_write!(t, data, Consequence::WriteScratchpad, addr);
    assert_eq!(t.mem_cmd_offset, 0, "Processed offset is not right");
    assert_eq!(t.mem_cmd_size, 1, "Processed size is not right");
}

#[test]
#[allow(clippy::identity_op)]
fn write_logical_non_addressed() {
    let cmd_byte = 0
        | 1 << 5 // Perform a memory command
        | 2 << 1 // Addressed by logical address
        | 1 << 0; // Operation: Write
    let addr = [5, 6];

    let data = vec![START_BYTE, cmd_byte, 6, 7, 0, 1, 0xAA];
    test_logical_write!(t, data, Consequence::None, addr);
    assert_eq!(t.mem_cmd_offset, 0, "Processed offset is not right");
    assert_eq!(t.mem_cmd_size, 1, "Processed size is not right");
}

#[test]
#[allow(clippy::identity_op)]
fn read_logical_addressed_0_length() {
    let cmd_byte = 0
        | 1 << 5 // Perform a memory command
        | 2 << 1 // Addressed by logical address
        | 0 << 0; // Operation: Read
    let addr = [5, 6];

    let data = vec![START_BYTE, cmd_byte, 5, 6, 0, 0];

    new_transceiver_in_sync!(t);
    t.logical_address = addr;
    for b in &data {
        test_rx_no_response!(t, *b);
    }

    let crc = CRC8Autosar::new().update_move(&data);

    // Clock in the header CRC
    let header_crc = crc.clone().finalize();
    test_state!(t, State::MEMHeaderCRC);
    let crc = crc.update_single_move(header_crc);

    // Transmit the header CRC and check the CRC that comes back
    test_tx!(t, header_crc, crc.finalize());
    test_state!(t, State::WaitForStart);
}

#[test]
#[allow(clippy::identity_op)]
fn read_logical_addressed_1_length() {
    let cmd_byte = 0
        | 1 << 5 // Perform a memory command
        | 2 << 1 // Addressed by logical address
        | 0 << 0; // Operation: Read
    let addr = [5, 6];

    let data = vec![START_BYTE, cmd_byte, 5, 6, 0, 1];

    new_transceiver_in_sync!(t);
    t.logical_address = addr;
    for b in &data {
        test_rx_no_response!(t, *b);
    }

    let crc = CRC8Autosar::new().update_move(&data);

    // Clock in the header CRC
    let header_crc = crc.clone().finalize();
    test_state!(t, State::MEMHeaderCRC);
    test_tx!(t, header_crc, 0x00); // We expect a 0 back as payload
    let crc = crc.update_move(&[header_crc, 0x00]);

    // Check that we receive the correct CRC back
    test_state!(t, State::SendCRC);
    test_tx!(t, crc.finalize());
    test_state!(t, State::WaitForStart);
}

#[test]
#[allow(clippy::identity_op)]
fn read_logical_addressed_2_length() {
    let cmd_byte = 0
        | 1 << 5 // Perform a memory command
        | 2 << 1 // Addressed by logical address
        | 0 << 0; // Operation: Read
    let addr = [5, 6];

    let data = vec![START_BYTE, cmd_byte, 5, 6, 0, 2];

    new_transceiver_in_sync!(t);
    t.logical_address = addr;
    for b in &data {
        test_rx_no_response!(t, *b);
    }

    let crc = CRC8Autosar::new().update_move(&data);

    // Clock in the header CRC
    let header_crc = crc.clone().finalize();
    test_state!(t, State::MEMHeaderCRC);
    test_tx!(t, header_crc, 0x00); // We expect a 0 back as payload

    // Test the second byte
    test_state!(t, State::MEMTxPayload);
    test_tx!(t, 0x00);
    let crc = crc.update_move(&[header_crc, 0x00, 0x00]);

    // Check that we receive the correct CRC back
    test_state!(t, State::SendCRC);
    test_tx!(t, crc.finalize());
    test_state!(t, State::WaitForStart);
}

#[test]
#[allow(clippy::identity_op)]
fn read_logical_not_addressed_0_length() {
    let cmd_byte = 0
        | 1 << 5 // Perform a memory command
        | 2 << 1 // Addressed by logical address
        | 0 << 0; // Operation: Read
    let addr = [5, 6];

    let data = vec![START_BYTE, cmd_byte, 7, 8, 0, 0];

    new_transceiver_in_sync!(t);
    t.logical_address = addr;
    for b in &data {
        test_rx_no_response!(t, *b);
    }

    let crc = CRC8Autosar::new().update_move(&data);

    // Clock in the header CRC
    let header_crc = crc.clone().finalize();
    test_state!(t, State::MEMHeaderCRC);
    let crc = crc.update_move(&[header_crc, 0x00]);

    // Transmit the header CRC, after that the transceiver should immediately
    // wait for the final crc
    test_rx_no_response!(t, header_crc);
    test_state!(t, State::WaitForCRC);
    test_consequence!(t, Consequence::None);

    // And the CRC the other slave sends
    test_rx_no_response!(t, crc.finalize());
    test_state!(t, State::WaitForStart);
}

#[test]
#[allow(clippy::identity_op)]
fn read_logical_not_addressed_1_length() {
    let cmd_byte = 0
        | 1 << 5 // Perform a memory command
        | 2 << 1 // Addressed by logical address
        | 0 << 0; // Operation: Read
    let addr = [5, 6];

    let data = vec![START_BYTE, cmd_byte, 7, 8, 0, 1];

    new_transceiver_in_sync!(t);
    t.logical_address = addr;
    for b in &data {
        test_rx_no_response!(t, *b);
    }

    let crc = CRC8Autosar::new().update_move(&data);

    // Clock in the header CRC
    let header_crc = crc.clone().finalize();
    test_state!(t, State::MEMHeaderCRC);
    let crc = crc.update_move(&[header_crc, 0x00]);

    // Transmit the header CRC
    test_rx_no_response!(t, header_crc);
    test_state!(t, State::MEMRxPayload);

    // The transceiver receives the byte from the other slave
    // with no consequence, as it is not addressed
    test_rx_no_response!(t, 0x00);
    test_state!(t, State::WaitForCRC);
    test_consequence!(t, Consequence::None);

    // And the CRC the other slave sends
    test_rx_no_response!(t, crc.finalize());
    test_state!(t, State::WaitForStart);
}
