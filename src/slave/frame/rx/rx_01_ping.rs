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

use super::{OwnedStructReceiver, OwnedStructReceiverResult, RXType};

#[derive(Debug, PartialEq)]
pub struct RX01Ping {
    receiver: OwnedStructReceiver<Ping>,
}

#[repr(align(1))]
#[derive(Debug, Default, PartialEq)]
struct Ping {
    dst: [u8; 6],
    src: [u8; 6],
}

impl Receiver for RX01Ping {
    fn rx(self, data: u8, core: &mut Core, _callbacks: &mut Callbacks) -> Response {
        core.crc.update_single(data);

        match self.receiver.rx(data) {
            OwnedStructReceiverResult::Continue(receiver) => {
                Response::from_state(Self { receiver }.into())
            }
            OwnedStructReceiverResult::Done(v) => {
                if v.dst == core.my_mac {
                    State::WaitForCRC(Some(TXType::Ping(TX01Ping::new(v.src, core)))).into()
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

impl Default for RX01Ping {
    fn default() -> Self {
        Self {
            receiver: OwnedStructReceiver::new(Ping::default()),
        }
    }
}
