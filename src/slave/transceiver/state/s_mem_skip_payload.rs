use crate::slave::transceiver::Consequence;

use super::{super::Transceiver, State};

pub fn state_memory_skip_payload(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        t.pos += 1;

        if t.pos >= t.mem_length {
            t.consequence = Consequence::None;
            t.state = State::Crc;
            t.pos = 0;
        }
    }

    None
}
