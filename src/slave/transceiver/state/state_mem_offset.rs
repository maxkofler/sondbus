use crate::slave::transceiver::{state::State, Transceiver};

pub fn state_mem_offset(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        let long_offset = t.cur_cmd.mem_has_long_offset();

        let mut value = t.mem_cmd_offset.to_le_bytes();
        value[if long_offset { 1 - t.pos } else { t.pos } as usize] = rx;
        t.mem_cmd_offset = u16::from_le_bytes(value);

        t.pos += 1;

        if !(long_offset && t.pos < 2) {
            t.pos = 0;
            t.state = State::MEMSize;
        }
    }

    None
}
