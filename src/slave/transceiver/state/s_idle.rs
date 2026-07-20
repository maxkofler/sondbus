use crate::{
    crc8::CRC,
    model::command::{Command, ManagementCommand},
    slave::transceiver::Consequence,
};

use super::{super::Transceiver, State};

const MASK_COMMAND: u8 = 0b11_1111;
const MASK_SEQUENCE: u8 = 0b1100_0000;

pub fn state_idle(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.crc.reset();
        t.update_crc(rx);

        // Unpack the command and sequence from the received byte
        let command = rx & MASK_COMMAND;
        let sequence = (rx & MASK_SEQUENCE) >> 6;

        // If the sequence numbers don't match up and we're already
        // in sync, we've lost something and we loose sync with the bus
        if (t.sequence_no + 1) & 0b11 != sequence && t.in_sync {
            t.loose_sync();
            t.state = State::Idle;
            return None;
        }

        // Increment the sequence number by one to
        // the next one we expect
        t.sequence_no = (t.sequence_no + 1) & 0b11;

        let command = Command(command);

        t.cur_cmd = command.clone();

        // Reset some state variables
        t.consequence = Consequence::None;
        t.pos = 0;
        t.mem_length = 0;
        t.mem_offset = 0;

        let cmd = match t.cur_cmd.get_manangement_command() {
            Err(_) => {
                t.loose_sync();
                t.state = State::Idle;
                return None;
            }
            Ok(v) => v,
        };

        let state = if let Some(cmd) = cmd {
            match cmd {
                ManagementCommand::Nop => State::Crc,
                ManagementCommand::Sync => State::ManagementSync,
            }
        } else {
            match command.mem_slave_address_octets() {
                0 => State::MemoryOffset,
                _ => State::MemorySlaveAddress,
            }
        };

        // If we are NOT in sync, there is only one allowed
        // next state: sync, otherwise, we'll go back to idle
        // as we are not in sync with the bus and the data we
        // receive might be garbled
        let state = if !t.in_sync && state != State::ManagementSync {
            State::Idle
        } else {
            state
        };

        t.state = state;
    }

    None
}
