use crate::{
    slave::transceiver::{state::State, Consequence, Transceiver},
    test_log, PROTOCOL_VERSION_1, SYNC_SEQUENCE,
};

pub fn state_sync(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.update_crc(rx);

        if t.pos <= 14 {
            if rx != SYNC_SEQUENCE[t.pos as usize] {
                test_log!("Lost sync!");
                t.loose_sync();
                t.state = State::WaitForStart;
            }
        } else if t.pos >= 15 {
            let version = rx;
            if version != PROTOCOL_VERSION_1 {
                t.loose_sync();
            } else {
                t.consequence = Consequence::GainSync;
            }
            t.state = State::WaitForCRC;
        }

        t.pos += 1;
    }

    None
}
