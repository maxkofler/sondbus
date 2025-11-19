use crate::{
    crc8::CRC,
    slave::transceiver::{state::State, CallbackAction, Consequence, Transceiver},
};

pub fn state_wait_for_crc(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        t.state = if t.crc.finalize() == rx {
            handle_consequence(t);

            State::WaitForStart
        } else {
            // If we do not match the CRC, we loose sync
            // with the bus and go back to idle
            t.loose_sync();
            State::WaitForStart
        };
    }

    None
}

fn handle_consequence(t: &mut Transceiver) {
    match t.consequence {
        // No consequence, just do nothing
        Consequence::None => {}

        // Gain sync, latch the sequence number of the
        // sync command into the internal sync register
        // and go into the synchronized state
        Consequence::GainSync => {
            t.in_sync = true;
            t.sequence_no = (t.cur_cmd.raw() >> 6) & 0b11;
        }

        // Write the contents of the scratchpad to memory
        Consequence::WriteScratchpad => {
            let res = (t.callback)(CallbackAction::WriteMemory {
                offset: t.mem_cmd_offset,
                data: &mut t.scratchpad[0..t.mem_cmd_size as usize],
            });

            // If the callback was not successful
            // we loose sync with the bus, as an
            // illegal operation took place
            if res.is_err() {
                t.loose_sync();
                t.state = State::WaitForStart;
            }
        }
    }
}
