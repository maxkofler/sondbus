use crate::{
    crc8::CRC,
    impl_receiver_nop,
    slave::frame::{core::Core, sdo::SDOAbort, state::State, Response, Sender},
    Callbacks,
};

use super::{StructSender, StructSenderResult, TXType};

#[derive(Debug, PartialEq)]
pub struct TX1FSDOAbort {
    structure: SDOAbort,
    sender: StructSender,
}

impl From<TX1FSDOAbort> for State {
    fn from(value: TX1FSDOAbort) -> Self {
        Self::HandleTX(TXType::SDOAbort(value))
    }
}

impl_receiver_nop!(TX1FSDOAbort);

impl Sender for TX1FSDOAbort {
    fn tx(self, core: &mut Core, _callbacks: &mut Callbacks) -> Response {
        let (state, response) = self.sender.tx(&self.structure);
        core.crc.update_single(response);

        let response = Some(response);

        match state {
            StructSenderResult::Continue(sender) => Response {
                response,
                state: Self {
                    structure: self.structure,
                    sender,
                }
                .into(),
            },
            StructSenderResult::Done() => Response {
                response,
                state: State::SendResponseCRC,
            },
        }
    }
}

impl TX1FSDOAbort {
    pub fn new(abort: SDOAbort) -> Self {
        Self {
            structure: abort,
            sender: StructSender::default(),
        }
    }
}
