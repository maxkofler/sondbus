use crate::{
    crc8::CRC,
    slave::frame::{core::Core, state::State, Receiver, Response},
    Callbacks, SYNC_SEQUENCE,
};

use super::{OwnedStructReceiver, OwnedStructReceiverResult, RXType};

#[derive(Debug, PartialEq)]
pub struct RX00Sync {
    receiver: OwnedStructReceiver<[u8; 15]>,
}

impl Receiver for RX00Sync {
    fn rx(self, data: u8, core: &mut Core, _callbacks: &mut Callbacks) -> Response {
        core.crc.update_single(data);

        match self.receiver.rx(data) {
            OwnedStructReceiverResult::Continue(receiver) => {
                Response::from_state(RX00Sync { receiver }.into())
            }
            OwnedStructReceiverResult::Done(sync) => {
                if sync == SYNC_SEQUENCE {
                    core.in_sync = true;
                }
                State::WaitForCRC(None).into()
            }
        }
    }
}

impl From<RX00Sync> for State {
    fn from(value: RX00Sync) -> Self {
        Self::HandleRX(RXType::Sync(value))
    }
}

impl Default for RX00Sync {
    fn default() -> Self {
        Self {
            receiver: OwnedStructReceiver::new([0u8; 15]),
        }
    }
}
