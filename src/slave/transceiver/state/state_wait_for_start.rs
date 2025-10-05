use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::transceiver::{state::State, Consequence, Transceiver},
};

pub fn state_wait_for_start(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        if rx == 0x55 {
            t.state = State::WaitForCommand;
            t.consequence = Consequence::None;
            t.crc = CRC8Autosar::new_const().update_single_move(rx);
            t.pos = 0;
        }
    }

    None
}
