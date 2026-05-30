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

        t.state = if t.is_targeted() {
            if t.cur_cmd.is_memory_read() && t.mem_length > 0 {
                // Call the callback to fill our scratchpad here
                let res = (t.callback)(CallbackAction::ReadMemory {
                    offset: t.mem_offset,
                    data: &mut t.scratchpad[..(t.mem_length as usize)],
                });

                // If the scratchpad read failed, we loose sync and return
                // to idle
                if res.is_err() {
                    t.loose_sync();
                    test_log!("Callback READ function returned error!");
                    t.state = State::Idle;
                    return None;
                }

                State::MemoryTXPayload
            } else {
                State::MemoryRXPayload
            }
        } else {
            State::MemorySkipPayload
        };

        /*
        if let Command::Memory(m) = &t.cur_cmd {
            if m.operation == Operation::Read && t.mem_length > 0 {

                t.state = State::MemoryTXPayload;
            }
        }
        */

        // If we expect no data (length=0), we skip directly to the CRC
        if t.mem_length == 0 {
            t.state = State::Crc;
        }
    }

    None
}
