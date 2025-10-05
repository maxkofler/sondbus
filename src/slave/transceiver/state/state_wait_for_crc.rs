use crate::{
    crc8::CRC,
    slave::transceiver::{state::State, Consequence, Transceiver},
};

pub fn state_wait_for_crc(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.state = if t.crc.finalize() == rx {
            // TODO: Handle the consequence

            match t.consequence {
                Consequence::GainSync => {
                    t.in_sync = true;
                    t.sequence_no = (t.cur_cmd.raw() >> 6) & 0b11;
                }
                _ => {}
            }

            State::WaitForStart
        } else {
            // If we do not match the CRC, we loose sync
            // with the bus and go back to idle
            t.loose_sync();
            State::WaitForStart
        }
    }

    None
}
