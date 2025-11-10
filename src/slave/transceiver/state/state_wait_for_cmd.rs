use crate::{
    slave::transceiver::{command::Command, state::State, Transceiver},
    CMD_NOP, CMD_SYNC,
};

const MASK_CMD_COMMAND: u8 = 0b11_1111;
const MASK_CMD_SEQUENCE: u8 = 0b1100_0000;

pub fn state_wait_for_cmd(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        // Unpack the command and sequence from the received byte
        let cmd = rx & MASK_CMD_COMMAND;
        let seq = (rx & MASK_CMD_SEQUENCE) >> 6;

        // If the sequence numbers don't match up and we're already
        // in sync, we've lost something and we loose sync with the bus
        if (t.sequence_no + 1) & 0b11 != seq && t.in_sync {
            t.loose_sync();
            t.state = State::WaitForStart;
            return None;
        }

        // Increment the sequence number by one to
        // the next one we expect
        t.sequence_no = (t.sequence_no + 1) & 0b11;

        // Set the internal command for later use
        t.cur_cmd = Command::new(rx);

        // Match on the command to determine which state to transition to.
        match cmd {
            CMD_NOP => t.state = State::WaitForCRC,
            CMD_SYNC => t.state = State::Sync,
            _ => {
                // An unknown command has been received.
                // In that case, we loose sync and go back to idle
                t.loose_sync();
                t.state = State::WaitForStart
            }
        }
    }

    None
}
