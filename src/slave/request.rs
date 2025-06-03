use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::{BusState, ResponseState, SlaveCore},
};

#[derive(PartialEq, Debug)]
pub enum CommandRequest {
    // NOTE: The NOP command request state does not really exist,
    //       as the bus immediately skips the payload step and
    //       enters the CRC state. This is correct-by-design
    //       as it makes the rx() call in the NOP state un-representable
}

#[derive(PartialEq, Debug)]
pub enum RequestState {
    /// Wait for the command byte to be received
    Command(CRC8Autosar),
    /// Wait for and handle the payload of the command
    Payload(CommandRequest),
    /// Wait for the CRC to be received
    CRC(Option<ResponseState>, u8),
}

impl RequestState {
    /// Handle an incoming byte from the bus
    /// # Arguments
    /// * `data` - The byte of data to process
    /// # Returns
    /// A tuple of the next bus state and a possible response byte
    pub fn rx(self, data: u8, core: &mut SlaveCore) -> (BusState, Option<u8>) {
        match self {
            //
            // Wait for the incoming command byte and handle it,
            // parsing the value and if it is not valid, kicking the
            // bus out of sync and going back to idle
            //
            Self::Command(crc) => {
                let command = CommandRequest::from_u8(data, crc);

                match command {
                    None => {
                        core.in_sync = false;
                        (BusState::Idle, None)
                    }
                    Some(state) => (BusState::Request(state), None),
                }
            }

            //
            //
            //
            Self::Payload(state) => state.rx(data, core),

            //
            //
            //
            Self::CRC(response, crc) => {
                if crc == data {
                    if let Some(response) = response {
                        BusState::ResponseStart(response).tx()
                    } else {
                        (BusState::Idle, None)
                    }
                } else {
                    core.in_sync = false;
                    (BusState::Idle, None)
                }
            }
        }
    }
}

impl CommandRequest {
    fn rx(self, data: u8, core: &mut SlaveCore) -> (BusState, Option<u8>) {
        (BusState::Idle, None)
    }

    fn from_u8(data: u8, crc: CRC8Autosar) -> Option<RequestState> {
        let crc = crc.update_single_move(data);

        match data {
            //
            // In the case of a NOP command, we immediately skip forward
            // to the CRC, skipping the payload, as there is none
            //
            0x00 => Some(RequestState::CRC(None, crc.finalize())),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {}
