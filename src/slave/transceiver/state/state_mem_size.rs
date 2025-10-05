use crate::slave::transceiver::{state::State, Transceiver};

pub fn state_mem_size(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        let long_size = t.cur_cmd.mem_has_long_size();

        let mut value = t.mem_cmd_size.to_le_bytes();
        value[if long_size { 1 - t.pos } else { t.pos } as usize] = rx;
        t.mem_cmd_size = u16::from_le_bytes(value);

        t.pos += 1;

        if !(long_size && t.pos < 2) {
            t.pos = 0;
            t.state = if t.mem_cmd_size > 0 {
                if t.cur_cmd.mem_is_write_cmd() {
                    State::MEMRxPayload
                } else {
                    State::MEMHeaderCRC
                }
            } else if t.cur_cmd.mem_is_read_cmd() {
                State::MEMHeaderCRC
            } else {
                State::WaitForCRC
            };
        }
    }

    None
}
