use crate::slave::transceiver::{state::State, Transceiver};

pub fn state_mem_tx_payload(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    // We NEVER expect any data
    if rx.is_some() {
        t.loose_sync();
        None
    } else {
        let tx_data = t.scratchpad[t.pos as usize];
        t.update_crc(tx_data);

        t.pos += 1;
        if t.pos >= t.mem_cmd_size {
            t.state = State::SendCRC;
        } else {
            t.state = State::MEMTxPayload;
        }

        Some(tx_data)
    }
}
