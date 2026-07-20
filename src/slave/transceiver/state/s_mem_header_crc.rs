use crate::{crc8::CRC, slave::transceiver::CallbackAction, test_log};

use super::{super::Transceiver, State};

pub fn state_memory_header_crc(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        // Assert that the CRC matches
        if rx != t.crc.finalize() {
            t.loose_sync();
            t.state = State::Idle;
            return None;
        }

        // Update the CRC only after we asserted it is ok
        t.update_crc(rx);

        let (state, res) = if t.cur_cmd.is_memory_read() {
            handle_read(t)
        } else {
            handle_write(t)
        };

        t.state = state;
        return res;
    }

    None
}

fn handle_write(t: &mut Transceiver) -> (State, Option<u8>) {
    let state = match t.mem_length {
        0 => State::Crc,
        _ => match t.is_targeted() {
            false => State::MemorySkipPayload,
            true => State::MemoryRXPayload,
        },
    };

    (state, None)
}

fn handle_read(t: &mut Transceiver) -> (State, Option<u8>) {
    // If we are targeted, call the callback and read the memory
    if t.is_targeted() && t.mem_length > 0 {
        let res = (t.callback)(CallbackAction::ReadMemory {
            offset: t.mem_offset,
            data: &mut t.scratchpad[..(t.mem_length as usize)],
        });

        // If the scratchpad read failed, we loose sync and return
        // to idle
        if res.is_err() {
            t.loose_sync();
            test_log!("Callback READ function returned error!");
            return (State::Idle, None);
        }
    }

    match t.is_targeted() {
        // If we are not targeted, we skip the payload
        // or skip to the Crc if no data is to be transmitted
        false => match t.mem_length {
            0 => (State::Crc, None),
            _ => (State::MemorySkipPayload, None),
        },
        // If we are targeted, start writing out the payload
        // or send the CRC and go to Idle if no data is to be transmitted
        true => match t.mem_length {
            0 => (State::Idle, Some(t.crc.finalize())),
            1 => (State::TxCrc, {
                let v = t.scratchpad[0];
                t.update_crc(v);
                Some(v)
            }),
            _ => (State::MemoryTXPayload, {
                let v = t.scratchpad[0];
                t.pos += 1;
                t.update_crc(v);
                Some(v)
            }),
        },
    }
}
