use crate::slave::transceiver::{state::State, Consequence, Transceiver};

pub fn state_mem_rx_payload(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        t.scratchpad[t.pos as usize] = rx;
        t.pos += 1;

        if t.pos >= t.mem_cmd_size {
            t.pos = 0;
            t.state = State::WaitForCRC;

            // If we are targeted, write the scatchpad contents,
            // otherwise we do nothing and simply wait for the CRC
            t.consequence = if t.is_targeted() {
                Consequence::WriteScratchpad
            } else {
                Consequence::None
            }
        }
    }

    None
}
