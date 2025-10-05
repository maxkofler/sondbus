use crate::{
    crc8::CRC,
    slave::transceiver::{state::State, Transceiver},
};

pub fn state_send_crc(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    // In either way we end up in the idle state after this state
    t.state = State::WaitForStart;

    // We NEVER expect any data
    if rx.is_some() {
        t.loose_sync();
        None
    } else {
        Some(t.crc.finalize())
    }
}
