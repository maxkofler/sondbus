use crate::{
    crc8::CRC,
    slave::frame::{core::Core, sdo::SDOAbort, state::State, Receiver, Response},
    Callbacks,
};

use super::{OwnedStructReceiver, OwnedStructReceiverResult, RXType};

#[derive(Debug, PartialEq)]
pub struct RX1FSDOAbort {
    receiver: OwnedStructReceiver<SDOAbort>,
}

impl Receiver for RX1FSDOAbort {
    fn rx(self, data: u8, core: &mut Core, _callbacks: &mut Callbacks) -> Response {
        core.crc.update_single(data);

        match self.receiver.rx(data) {
            OwnedStructReceiverResult::Continue(receiver) => {
                Response::from_state(Self { receiver }.into())
            }
            OwnedStructReceiverResult::Done(_) => Response {
                response: None,
                state: State::WaitForCRC(None),
            },
        }
    }
}

impl From<RX1FSDOAbort> for State {
    fn from(value: RX1FSDOAbort) -> Self {
        Self::HandleRX(RXType::SDOAbort(value))
    }
}

impl Default for RX1FSDOAbort {
    fn default() -> Self {
        Self {
            receiver: OwnedStructReceiver::new(SDOAbort::default()),
        }
    }
}
