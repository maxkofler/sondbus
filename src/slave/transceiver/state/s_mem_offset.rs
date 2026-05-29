use super::{super::Transceiver, State};

pub fn state_memory_offset(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        t.mem_offset = rx as u64;
        t.state = State::MemoryLength;
    }

    None
}
