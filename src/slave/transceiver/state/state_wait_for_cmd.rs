use crate::{
    slave::transceiver::{
        command::{AddressingMode, Command},
        state::State,
        Transceiver,
    },
    test_log, CMD_NOP, CMD_SYNC,
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
        let state = match cmd {
            CMD_NOP => State::WaitForCRC,
            CMD_SYNC => State::Sync,
            0b1_00000..0b1_11111 => handle_mem_cmd(t),
            _ => {
                // An unknown command has been received.
                // In that case, we loose sync and go back to idle
                t.loose_sync();
                State::WaitForStart
            }
        };

        // If we are NOT in sync, there is only one allowed
        // next state: sync, otherwise, we'll go back to idle
        // as we are not in sync with the bus and the data we
        // receive might be garbled
        let state = if !t.in_sync && state != State::Sync {
            State::WaitForStart
        } else {
            state
        };

        t.state = state;
    }

    None
}

fn handle_mem_cmd(t: &mut Transceiver) -> State {
    t.pos = 0;
    test_log!(
        "Addressing mode: {:?}",
        t.cur_cmd.mem_slave_addressing_mode()
    );
    match t.cur_cmd.mem_slave_addressing_mode() {
        AddressingMode::Broadcast | AddressingMode::None => State::MEMOffset,
        AddressingMode::Physical | AddressingMode::Logical => State::MEMAddress,
    }
}
