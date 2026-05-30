use crate::{crc8::CRC, slave::transceiver::CallbackAction, test_log};

use super::{super::Consequence, State, Transceiver};

pub fn state_crc(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        if rx != t.crc.finalize() {
            t.loose_sync();
        } else {
            handle_consequence(t);
        }

        t.state = State::Idle;
    }

    None
}

fn handle_consequence(t: &mut Transceiver) {
    match t.consequence {
        Consequence::None => {}
        Consequence::GainSync => {
            test_log!("Gained sync!");
            t.in_sync = true;
        }
        Consequence::WriteScratchpad => {
            if (t.callback)(CallbackAction::WriteMemory {
                offset: t.mem_offset,
                data: &t.scratchpad[..t.mem_length as usize],
            })
            .is_err()
            {
                t.loose_sync();
            }
        }
    }
}
