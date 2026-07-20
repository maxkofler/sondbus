use crate::crc8::CRC;

use super::{super::Transceiver, State};

pub fn state_tx_crc(t: &mut Transceiver, _rx: Option<u8>) -> Option<u8> {
    // TODO: Handle rx here... Should probably assert that nothing
    // is received in this phase... But for deferred mode, this may
    // be different...

    t.state = State::Idle;

    Some(t.crc.finalize())
}
