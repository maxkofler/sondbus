use crate::{
    crc8::CRC,
    impl_receiver_nop,
    slave::frame::{core::Core, state::State, Response, Sender},
    Callbacks,
};

use super::{StructSender, StructSenderResult, TXType};

#[derive(Debug, PartialEq)]
pub struct TX01Ping {
    structure: Ping,
    sender: StructSender,
}

#[derive(Debug, PartialEq)]
struct Ping {
    dst: [u8; 6],
    src: [u8; 6],
}

impl From<TX01Ping> for State {
    fn from(value: TX01Ping) -> Self {
        Self::HandleTX(TXType::Ping(value))
    }
}

impl_receiver_nop!(TX01Ping);

impl Sender for TX01Ping {
    fn tx(self, core: &mut Core, _callbacks: &mut Callbacks) -> Response {
        let (state, response) = self.sender.tx(&self.structure);

        core.crc.update_single(response);

        match state {
            StructSenderResult::Continue(sender) => Response {
                state: State::HandleTX(TXType::Ping(Self {
                    structure: self.structure,
                    sender,
                })),
                response: Some(response),
            },
            StructSenderResult::Done() => Response {
                state: State::SendResponseCRC,
                response: Some(response),
            },
        }
    }
}

impl TX01Ping {
    pub fn new(dst: [u8; 6], core: &Core) -> Self {
        let ping = Ping {
            dst,
            src: core.my_mac,
        };

        Self {
            structure: ping,
            sender: StructSender::default(),
        }
    }
}
