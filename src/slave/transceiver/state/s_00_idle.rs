use crate::model::command::{Command, ManagementCommand};

use super::{super::Transceiver, State};

const MASK_COMMAND: u8 = 0b11_1111;
const MASK_SEQUENCE: u8 = 0b1100_0000;

pub fn state_idle(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
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

        let command = match Command::try_from(command) {
            Ok(v) => v,
            Err(_) => {
                t.loose_sync();
                t.state = State::Idle;
                return None;
            }
        };

        t.cur_cmd = command.clone();

        let state = match command {
            Command::Management(c) => match c {
                ManagementCommand::Nop => State::CRC,
                _ => State::Idle,
            },
            _ => State::Idle,
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
