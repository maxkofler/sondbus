use crate::SYNC_SEQUENCE;

use super::{super::Consequence, super::Transceiver, State};

pub fn state_management_sync(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        if t.pos < 14 {
            if rx != SYNC_SEQUENCE[t.pos as usize] {
                t.loose_sync();
                t.state = State::Idle;
            }
        } else {
            t.consequence = Consequence::GainSync;
            t.state = State::CRC;
        }

        t.pos += 1;
    }

    None
}
