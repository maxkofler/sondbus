use crate::{
    crc8::{CRC8Autosar, CRC},
    FrameType, START_BYTE,
};

use super::{core::Core, Receiver, Response};

#[derive(Default, Debug)]
pub enum State {
    /// Wait for the start byte to be received before
    /// proceeding to the next state
    #[default]
    WaitForStart,
    /// Waits for the type of frame being transmitted
    WaitForType,
    /// Handle request data coming in from the bus
    HandleRX,
    /// Waits for the CRC and proceeds to handling the
    /// response if it is valid
    WaitForCRC,
    /// Handles the response to be put on the bus
    HandleTX,
}

impl Receiver for State {
    fn rx(self, data: u8, core: &mut Core) -> Response {
        match self {
            // Wait for the start byte to arrive at the bus
            // and transition to the next state.
            // If the byte is not correct, remain in the current state
            State::WaitForStart => {
                if data == START_BYTE {
                    core.crc = CRC8Autosar::default().update_single_move(START_BYTE);
                    Self::WaitForType.into()
                } else {
                    self.into()
                }
            }

            // Wait for the type byte and parse it,
            // if we receive a unknown byte, get out of
            // sync and return to the start state
            State::WaitForType => {
                core.crc.update_single(data);
                match FrameType::from_u8(data) {
                    Some(_) => Self::HandleRX.into(),
                    None => {
                        // If we receive an invalid type, we're out of sync
                        core.in_sync = false;
                        Self::WaitForStart.into()
                    }
                }
            }

            // Handle incoming bytes of a specific frame type
            State::HandleRX => Self::WaitForCRC.into(),

            // Wait for the CRC of the whole data.
            // If we have a CRC error, the sync is lost
            State::WaitForCRC => {
                if core.crc.finalize() == data {
                    State::HandleTX.into()
                } else {
                    // If we have a CRC error, we're out of sync
                    core.in_sync = false;
                    State::WaitForStart.into()
                }
            }

            // Send the response data to the bus
            State::HandleTX => Self::WaitForStart.into(),
        }
    }
}

impl From<State> for Response {
    fn from(state: State) -> Self {
        Self {
            state,
            response: None,
        }
    }
}
