use crate::{
    crc8::CRC,
    impl_receiver_nop,
    slave::frame::{core::Core, sdo::SDOAbort, state::State, Response, Sender},
    Callbacks,
};

use super::{OwnedStructSender, OwnedStructSenderResult, TXType};

#[derive(Debug, PartialEq)]
pub struct TX1FSDOAbort {
    sender: OwnedStructSender<SDOAbort>,
}

impl From<TX1FSDOAbort> for State {
    fn from(value: TX1FSDOAbort) -> Self {
        Self::HandleTX(TXType::SDOAbort(value))
    }
}

impl_receiver_nop!(TX1FSDOAbort);

impl Sender for TX1FSDOAbort {
    fn tx(self, core: &mut Core, _callbacks: &mut Callbacks) -> Response {
        let (state, response) = self.sender.tx();
        core.crc.update_single(response);

        let response = Some(response);

        match state {
            OwnedStructSenderResult::Continue(sender) => Response {
                response,
                state: Self { sender }.into(),
            },
            OwnedStructSenderResult::Done() => Response {
                response,
                state: State::SendResponseCRC,
            },
        }
    }
}

impl TX1FSDOAbort {
    pub fn new(abort: SDOAbort) -> Self {
        Self {
            sender: OwnedStructSender::new(abort),
        }
    }
}
