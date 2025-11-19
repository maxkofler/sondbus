use crate::{
    crc8::CRC,
    slave::transceiver::{state::State, CallbackAction, Transceiver},
};

pub fn state_mem_header_crc(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        if t.crc.finalize() == rx {
            // Call the application to fill the buffer for us
            let res = (t.callback)(CallbackAction::ReadMemory {
                offset: t.mem_cmd_offset,
                data: &mut t.scratchpad[0..(t.mem_cmd_size as usize)],
            });

            // Check the return code. If it is not ok,
            // we loose sync with the bus, as an illegal
            // operation was performed
            if res.is_err() {
                t.loose_sync();
                t.state = State::WaitForStart;
                return None;
            }

            t.update_crc(rx);
            t.pos = 0;

            return if t.is_targeted() {
                handle_targeted(t)
            } else {
                handle_not_targeted(t)
            };
        } else {
            // If we do not match the CRC, we loose sync
            // with the bus and go back to idle
            t.loose_sync();
            t.state = State::WaitForStart;
        }
    }

    None
}

fn handle_targeted(t: &mut Transceiver) -> Option<u8> {
    match t.mem_cmd_size {
        // A zero-length command results in an immediate CRC
        0 => {
            t.state = State::WaitForStart;
            Some(t.crc.finalize())
        }
        // A one-length read results in the one byte and then the CRC
        1 => {
            t.state = State::SendCRC;
            let tx_data = t.scratchpad[0];
            t.update_crc(tx_data);
            Some(tx_data)
        }
        _ => {
            t.state = State::MEMTxPayload;
            let tx_data = t.scratchpad[0];
            t.update_crc(tx_data);
            t.pos = 1;
            Some(tx_data)
        }
    }
}

fn handle_not_targeted(t: &mut Transceiver) -> Option<u8> {
    t.state = match t.mem_cmd_size {
        0 => State::WaitForCRC,
        _ => State::MEMRxPayload,
    };

    None
}
