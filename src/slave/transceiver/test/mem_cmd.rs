use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::transceiver::{
        test::{new_transceiver, SCRATCHPAD},
        Consequence, State, Transceiver,
    },
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
        0x00,     // The CRC (to be set later)
    ];

    t.test_rx_multi_no_response(&data);

    data[3] = CRC8Autosar::new().update_move(&data[0..3]).finalize();

    t.test_rx_multi_no_response(&data);
}

impl Transceiver {
    fn test_write(&mut self, cmd_byte: u8, consequence: Consequence) {
        // The transceiver should now wait for a one byte offset
        self.test_state(State::MEMOffset);
        self.test_rx_no_response(1);

        // The transceiver should now wait for a one byte size
        self.test_state(State::MEMSize);
        self.test_rx_no_response(2);

        // A write command does not have a header CRC

        // We should now wait for exactly 2 bytes of payload
        for i in 0..2 {
            self.test_state(State::MEMRxPayload);
            self.test_rx_no_response(i);
        }

        // And now we should wait for the final CRC
        self.test_state(State::WaitForCRC);
        assert_eq!(self.consequence, consequence);
        let crc = CRC8Autosar::new()
            .update_move(&[cmd_byte, 1, 2, 0, 1])
            .finalize();
        self.test_rx_no_response(crc);

        // And back to idle
        self.test_state(State::WaitForStart);
    }

    fn test_read(&mut self, mut crc: CRC8Autosar) {
        // The transceiver should now wait for a one byte offset
        self.test_state(State::MEMOffset);
        self.test_rx_no_response(1);

        // The transceiver should now wait for a one byte size
        self.test_state(State::MEMSize);
        self.test_rx_no_response(2);

        self.scratchpad[0] = 0;
        self.scratchpad[1] = 0;

        // Construct and send the header CRC
        self.test_state(State::MEMHeaderCRC);
        crc.update(&[1, 2]);
        assert_eq!(self.handle(Some(crc.finalize())).expect("TX Payload"), 0);
        crc.update_single(crc.finalize());

        // We should now wait for exactly 2 bytes of payload
        self.test_state(State::MEMTxPayload);
        assert_eq!(self.handle(None).expect("TX Payload"), 0);

        println!("Sond: {:x}", crc.finalize());
        crc.update(&[0, 0]);

        // And now we should wait for the final CRC
        self.test_state(State::SendCRC);
        let res_crc = self
            .handle(None)
            .expect("SendCRC should result in a sent CRC");
        assert_eq!(
            crc.finalize(),
            res_crc,
            "Transceiver CRC is wrong, expected {:x}, got {:x}",
            crc.finalize(),
            res_crc
        );

        // And back to idle
        self.test_state(State::WaitForStart);
    }
}

/// Performs a broadcast write with a 1-byte offset and size
#[test]
fn test_broadcast_o1_s1_write() {
    let mut t = new_transceiver();

    t.test_rx_no_response(START_BYTE);

    let cmd_byte = 0 |
        1 << 5 | // Perform a memory command
        1 << 0; // Perform a write
    t.test_rx_no_response(cmd_byte);

    // Test the rest of the receive process
    t.test_write(cmd_byte, Consequence::WriteScratchpad);
}

/// Performs a physically addressed write with a 1-byte offset and size
#[test]
fn test_physical_o1_s1_write() {
    let mut t = new_transceiver();
    t.physical_address = [0u8; 6];

    t.test_rx_no_response(START_BYTE);

    let cmd_byte = 0 |
        1 << 5 | // Perform a memory command
        1 << 1 | // Addressed physically
        1 << 0; // Perform a write
    t.test_rx_no_response(cmd_byte);

    // Clock in the physical address of the slave
    for _ in 0..6 {
        t.test_state(State::MEMAddress);
        t.test_rx_no_response(0);
    }

    // Test the rest of the receive process
    t.test_write(cmd_byte, Consequence::WriteScratchpad);
}

/// Performs a physically addressed read with a 1-byte offset and size
#[test]
fn test_physical_o1_s1_read() {
    let mut t = new_transceiver();
    t.physical_address = [0u8; 6];

    t.test_rx_no_response(START_BYTE);

    let cmd_byte = 0 |
        1 << 5 | // Perform a memory command
        1 << 1 | // Addressed physically
        0 << 0; // Perform a read
    t.test_rx_no_response(cmd_byte);

    // Clock in the physical address of the slave
    for _ in 0..6 {
        t.test_state(State::MEMAddress);
        t.test_rx_no_response(0);
    }

    let crc = CRC8Autosar::new().update_move(&[START_BYTE, cmd_byte, 0, 0, 0, 0, 0, 0]);

    // Test the rest of the receive process
    t.test_read(crc);
}

/// Performs a logically addressed write with a 1-byte offset and size
#[test]
fn test_logical_o1_s1_write() {
    let mut t = new_transceiver();
    t.logical_address = [0u8; 2];

    t.test_rx_no_response(START_BYTE);

    let cmd_byte = 0 |
        1 << 5 | // Perform a memory command
        1 << 2 | // Addressed logically
        1 << 0; // Perform a write
    t.test_rx_no_response(cmd_byte);

    // Clock in the physical address of the slave
    for _ in 0..2 {
        t.test_state(State::MEMAddress);
        t.test_rx_no_response(0);
    }

    // Test the rest of the receive process
    t.test_write(cmd_byte, Consequence::WriteScratchpad);
}
