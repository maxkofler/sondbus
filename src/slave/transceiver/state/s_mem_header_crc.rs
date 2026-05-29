use crate::{
    model::command::{Command, Operation},
    slave::transceiver::CallbackAction,
};

use super::{super::Transceiver, State};

pub fn state_memory_header_crc(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        if let Command::Memory(m) = &t.cur_cmd {
            if m.operation == Operation::Read && t.mem_length > 0 {
                // Call the callback to fill our scratchpad here
                let res = (t.callback)(CallbackAction::ReadMemory {
                    offset: t.mem_offset as usize,
                    data: &mut t.scratchpad[..(t.mem_length as usize)],
                });

                // If the scratchpad read failed, we loose sync and return
                // to idle
                if res.is_err() {
                    t.loose_sync();
                    t.state = State::Idle;
                    return None;
                }

                t.state = State::MemoryTXPayload;
            }
        }

        // If we expect no data (length=0), we skip directly to the CRC
        if t.mem_length == 0 {
            t.state = State::CRC;
        }
    }

    None
}
