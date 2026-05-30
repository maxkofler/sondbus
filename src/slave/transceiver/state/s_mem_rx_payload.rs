use crate::slave::transceiver::Consequence;

use super::{super::Transceiver, State};

pub fn state_memory_rx_payload(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        t.scratchpad[t.pos as usize] = rx;
        t.pos += 1;

        if t.pos >= t.mem_length {
            t.consequence = Consequence::WriteScratchpad;
            t.state = State::Crc;
            t.pos = 0;
        }
    }

    None
}
