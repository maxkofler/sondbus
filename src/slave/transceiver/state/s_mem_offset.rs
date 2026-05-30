use super::{super::Transceiver, State};

pub fn state_memory_offset(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        let len_octets = t.cur_cmd.mem_memory_addressing_octets();
        t.mem_offset |= (rx as u64) << ((len_octets - 1) - t.pos) * 8;
        t.pos += 1;

        if t.pos >= len_octets {
            t.state = State::MemoryLength;
            t.pos = 0;
        }
    }

    None
}
