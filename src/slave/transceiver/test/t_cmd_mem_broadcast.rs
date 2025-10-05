use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::transceiver::{
        state::State,
        test::{
            new_transceiver_in_sync, test_consequence, test_rx_no_response, test_state, test_sync,
        },
        Consequence,
    },
    START_BYTE,
};

macro_rules! test_write {
    ($t: ident, $data: expr) => {
        let crc = CRC8Autosar::new().update_move(&$data).finalize();
        new_transceiver_in_sync!($t);
        for b in $data {
            test_rx_no_response!($t, b);
        }

        test_consequence!($t, Consequence::WriteScratchpad);
        test_rx_no_response!($t, crc);

        test_state!($t, State::WaitForStart);
        test_sync!($t, true);
    };
}

#[test]
#[allow(clippy::identity_op)]
fn write_o1_s1_complete() {
    let cmd_byte = 0
        | 1 << 5 // Perform a memory command
        | 1 << 0; // Operation: Write

    let data = vec![START_BYTE, cmd_byte, 0, 1, 0xAA];
    test_write!(t, data);
    assert_eq!(t.mem_cmd_offset, 0, "Processed offset is not right");
    assert_eq!(t.mem_cmd_size, 1, "Processed size is not right");
}

#[test]
#[allow(clippy::identity_op)]
fn write_o2_s1_complete() {
    let cmd_byte = 0
        | 1 << 5 // Perform a memory command
        | 1 << 3 // Long offset
        | 1 << 0; // Operation: Write

    let data = vec![START_BYTE, cmd_byte, 0x12, 0x34, 1, 0xAA];
    test_write!(t, data);
    assert_eq!(t.mem_cmd_offset, 0x1234, "Processed offset is not right");
    assert_eq!(t.mem_cmd_size, 1, "Processed size is not right");
}

#[test]
#[allow(clippy::identity_op)]
fn write_o1_s2_complete() {
    let cmd_byte = 0
        | 1 << 5 // Perform a memory command
        | 1 << 4 // Long size
        | 1 << 0; // Operation: Write

    let data = vec![START_BYTE, cmd_byte, 0xFF, 0, 1, 0xAA];
    test_write!(t, data);
    assert_eq!(t.mem_cmd_offset, 0xFF, "Processed offset is not right");
    assert_eq!(t.mem_cmd_size, 1, "Processed size is not right");
}

#[test]
#[allow(clippy::identity_op)]
fn write_o2_s2_complete() {
    let cmd_byte = 0
        | 1 << 5 // Perform a memory command
        | 1 << 3 // Long offset
        | 1 << 4 // Long size
        | 1 << 0; // Operation: Write

    let data = vec![START_BYTE, cmd_byte, 0x23, 0x45, 0, 1, 0xAA];
    test_write!(t, data);
    assert_eq!(t.mem_cmd_offset, 0x2345, "Processed offset is not right");
    assert_eq!(t.mem_cmd_size, 1, "Processed size is not right");
}

#[test]
#[allow(clippy::identity_op)]
fn write_o1_s1_zero_length() {
    let cmd_byte = 0
        | 1 << 5 // Perform a memory command
        | 1 << 0; // Operation: Write

    let data = vec![START_BYTE, cmd_byte, 0, 0];
    let crc = CRC8Autosar::new().update_move(&data).finalize();

    new_transceiver_in_sync!(t);
    for b in data {
        test_rx_no_response!(t, b);
    }

    // Since there is no data sent, we do not expect a
    // write scratchpad consequence
    test_consequence!(t, Consequence::None);
    test_rx_no_response!(t, crc);

    test_state!(t, State::WaitForStart);
    test_sync!(t, true);
}
