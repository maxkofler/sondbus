use crate::test_log;

use super::{super::Transceiver, State};

pub fn state_memory_length(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        let len_octets = t.cur_cmd.mem_length_octets();
        t.mem_length |= (rx as u64) << ((len_octets - 1) - t.pos) * 8;
        t.pos += 1;

        if t.pos >= t.cur_cmd.mem_length_octets() as u8 {
            test_log!("Mem length: {}", t.mem_length);
            t.state = State::MemoryHeaderCRC;
            t.pos = 0;
        }
    }

    None
}
