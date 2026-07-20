use super::{super::Transceiver, State};

pub fn state_memory_tx_payload(t: &mut Transceiver, _rx: Option<u8>) -> Option<u8> {
    // TODO: Handle rx here... Should probably assert that nothing
    // is received in this phase... But for deferred mode, this may
    // be different...

    let data = t.scratchpad[t.pos as usize];
    t.update_crc(data);
    t.pos += 1;

    if t.pos >= t.mem_length {
        t.state = State::TxCrc;
        t.pos = 0;
    }

    Some(data)
}
