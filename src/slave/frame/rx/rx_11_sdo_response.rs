use crate::{
    crc8::CRC,
    slave::frame::{core::Core, state::State, Receiver, Response},
    Callbacks,
};

use super::{OwnedStructReceiver, OwnedStructReceiverResult, RXType};

#[derive(Debug, PartialEq)]
pub struct RX11SDOResponse {
    receiver: OwnedStructReceiver<Ping>,
}

#[repr(align(1))]
#[derive(Debug, Default, PartialEq)]
struct Ping {
    universe: u8,
    address: u8,
    object_index: u16,
}

impl Receiver for RX11SDOResponse {
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

impl From<RX11SDOResponse> for State {
    fn from(value: RX11SDOResponse) -> Self {
        Self::HandleRX(RXType::SDOResponse(value))
    }
}

impl Default for RX11SDOResponse {
    fn default() -> Self {
        Self {
            receiver: OwnedStructReceiver::new(Ping::default()),
        }
    }
}
