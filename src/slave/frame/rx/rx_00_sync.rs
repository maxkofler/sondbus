use crate::{
    crc8::CRC,
    slave::frame::{core::Core, state::State, Receiver, Response},
    Callbacks, SYNC_SEQUENCE,
};

use super::{RXType, StructReceiver, StructReceiverResult};

#[derive(Debug, Default, PartialEq)]
pub struct RX00Sync {
    structure: [u8; 15],
    receiver: StructReceiver,
}

impl Receiver for RX00Sync {
    fn rx(mut self, data: u8, core: &mut Core, _callbacks: &mut Callbacks) -> Response {
        core.crc.update_single(data);

        match self.receiver.rx(data, &mut self.structure) {
            StructReceiverResult::Continue(receiver) => Response::from_state(
                RX00Sync {
                    structure: self.structure,
                    receiver,
                }
                .into(),
            ),
            StructReceiverResult::Done => {
                if self.structure == SYNC_SEQUENCE {
                    core.in_sync = true;
                } else {
                    panic!();
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
