use crate::test_log;

use super::{super::Transceiver, State};

pub fn state_memory_slave_address(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        t.mem_slave_addr[t.pos as usize] = rx;
        t.pos += 1;

        if t.pos >= t.cur_cmd.mem_address_octets() as u8 {
            test_log!(
                "Slave Address: {:?}",
                &t.mem_slave_addr[..t.cur_cmd.mem_address_octets() as usize]
            );
            t.state = State::MemoryOffset;
            t.pos = 0;
        }
    }

    None
}
