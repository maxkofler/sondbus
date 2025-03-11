use crate::{
    crc8::{CRC8Autosar, CRC},
    FrameType, START_BYTE,
};

use super::{core::Core, rx::RXType, tx::TXType, Receiver, Response, Sender};

#[derive(Default, Debug, PartialEq)]
pub enum State {
    /// Wait for the start byte to be received before
    /// proceeding to the next state
    #[default]
    WaitForStart,
    /// Waits for the type of frame being transmitted
    WaitForType,
    /// Handle request data coming in from the bus
    HandleRX(RXType),
    /// Waits for the CRC and proceeds to handling the
    /// response if it is valid
    WaitForCRC(Option<TXType>),
    SendResponseStart(TXType),
    SendResponseType(TXType),
    /// Handles the response to be put on the bus
    HandleTX(TXType),
    SendResponseCRC,
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
                    Some(v) => {
                        let state: State = RXType::from(v).into();
                        state.into()
                    }
                    None => {
                        // If we receive an invalid type, we're out of sync
                        core.in_sync = false;
                        Self::WaitForStart.into()
                    }
                }
            }

            // Handle incoming bytes of a specific frame type
            State::HandleRX(state) => state.rx(data, core),

            // Wait for the CRC of the whole data.
            // If we have a CRC error, the sync is lost
            State::WaitForCRC(r) => {
                let crc = core.crc.finalize();
                core.crc = Default::default();
                if crc == data {
                    match r {
                        Some(r) => State::SendResponseStart(r).tx(core),
                        None => State::WaitForStart.into(),
                    }
                } else {
                    // If we have a CRC error, we're out of sync
                    core.in_sync = false;
                    State::WaitForStart.into()
                }
            }

            // Send the response data to the bus
            State::HandleTX(v) => v.rx(data, core),

            _ => Response::from_state(self),
        }
    }
}

impl Sender for State {
    fn tx(self, core: &mut super::core::Core) -> Response {
        match self {
            State::SendResponseStart(v) => {
                core.crc.update_single(START_BYTE);
                Response {
                    state: State::SendResponseType(v),
                    response: Some(START_BYTE),
                }
            }

            State::SendResponseType(v) => {
                let ty = v.to_frame_type() as u8;
                core.crc.update_single(ty);
                Response {
                    state: State::HandleTX(v),
                    response: Some(ty),
                }
            }
            Self::HandleTX(v) => v.tx(core),
            State::SendResponseCRC => Response {
                state: Self::WaitForStart,
                response: Some(core.crc.finalize()),
            },
            _ => Response::from_state(self),
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
