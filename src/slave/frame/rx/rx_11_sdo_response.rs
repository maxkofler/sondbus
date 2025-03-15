use crate::{
    crc8::CRC,
    slave::frame::{core::Core, state::State, Receiver, Response},
    Callbacks,
};

use super::{RXType, StructReceiver, StructReceiverResult};

#[derive(Debug, Default, PartialEq)]
pub struct RX11SDOResponse {
    structure: Structure,
    receiver: StructReceiver,
}

#[repr(align(1))]
#[derive(Debug, Default, PartialEq)]
struct Structure {
    universe: u8,
    address: u8,
    object_index: u16,
}

impl Receiver for RX11SDOResponse {
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

impl From<RX11SDOResponse> for State {
    fn from(value: RX11SDOResponse) -> Self {
        Self::HandleRX(RXType::SDOResponse(value))
    }
}
