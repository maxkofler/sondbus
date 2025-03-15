use crate::{
    crc8::CRC,
    slave::frame::{
        core::Core,
        state::State,
        tx::{TX01Ping, TXType},
        Receiver, Response,
    },
    Callbacks,
};

use super::{RXType, StructReceiver, StructReceiverResult};

#[derive(Debug, Default, PartialEq)]
pub struct RX01Ping {
    structure: Ping,
    receiver: StructReceiver,
}

#[repr(align(1))]
#[derive(Debug, Default, PartialEq)]
struct Ping {
    dst: [u8; 6],
    src: [u8; 6],
}

impl Receiver for RX01Ping {
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
            StructReceiverResult::Done => {
                if self.structure.dst == core.my_mac {
                    State::WaitForCRC(Some(TXType::Ping(TX01Ping::new(self.structure.src, core))))
                        .into()
                } else {
                    State::WaitForCRC(None).into()
                }
            }
        }
    }
}

impl From<RX01Ping> for State {
    fn from(value: RX01Ping) -> Self {
        Self::HandleRX(RXType::Ping(value))
    }
}
