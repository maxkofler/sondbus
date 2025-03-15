use crate::{
    crc8::CRC,
    slave::frame::{core::Core, sdo::SDOAbort, state::State, Receiver, Response},
    Callbacks,
};

use super::{RXType, StructReceiver, StructReceiverResult};

#[derive(Debug, Default, PartialEq)]
pub struct RX1FSDOAbort {
    structure: SDOAbort,
    receiver: StructReceiver,
}

impl Receiver for RX1FSDOAbort {
    fn rx(mut self, data: u8, core: &mut Core, _callbacks: &mut Callbacks) -> Response {
        core.crc.update_single(data);

        match self.receiver.rx(data, &mut self.structure) {
            StructReceiverResult::Continue(receiver) => Response::from_state(
                Self {
                    structure: self.structure,
                    receiver,
                }
                .into(),
            ),
            StructReceiverResult::Done => Response {
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
