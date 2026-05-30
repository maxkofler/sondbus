use super::{super::Transceiver, State};

pub fn state_memory_length(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        t.mem_length = rx;
        t.state = State::MemoryHeaderCRC;
    }

    None
}
