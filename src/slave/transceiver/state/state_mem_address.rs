use crate::slave::transceiver::{state::State, Transceiver};

pub fn state_mem_address(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        t.mem_cmd_addr[t.pos as usize] = rx;
        t.pos += 1;

        if t.pos as u8 >= t.cur_cmd.mem_slave_address_len() {
            t.pos = 0;
            t.state = State::MEMOffset;
        }
    }

    None
}
