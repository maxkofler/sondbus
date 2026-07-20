use crate::{test_log, SYNC_SEQUENCE};

use super::{super::Consequence, super::Transceiver, State};

pub fn state_management_sync(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        match t.pos {
            0..15 => {
                if rx != SYNC_SEQUENCE[t.pos as usize] {
                    t.loose_sync();
                    t.state = State::Idle;
                }
            }
            // The last byte is the protocol version to be used
            15.. => {
                // TODO: Implement proper checks when spec is updated
                if rx != 1 {
                    test_log!("Version is not 1");
                    t.loose_sync();
                    t.state = State::Idle;
                } else {
                    t.consequence = Consequence::GainSync;
                    t.state = State::Crc;
                }
            }
        }

        t.pos += 1;
    }

    None
}
