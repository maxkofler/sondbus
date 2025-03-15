use crate::{
    crc8::CRC,
    impl_receiver_nop,
    slave::frame::{core::Core, state::State, Response, Sender},
    Callbacks, ObjectBuffer,
};

use super::{ArraySender, ArraySenderResult, TXType};

#[derive(Debug, PartialEq)]
pub enum TX11SDOResponse {
    Length(ObjectBuffer, u8),
    Data(ObjectBuffer, ArraySender),
}

impl From<TX11SDOResponse> for State {
    fn from(value: TX11SDOResponse) -> Self {
        Self::HandleTX(TXType::SDOResponse(value))
    }
}

impl_receiver_nop!(TX11SDOResponse);

impl Sender for TX11SDOResponse {
    fn tx(self, core: &mut Core, _callbacks: &mut Callbacks) -> Response {
        match self {
            Self::Length(buffer, len) => Response {
                response: Some(len),
                state: Self::Data(buffer, ArraySender::new_with_len(len as usize)).into(),
            },
            Self::Data(buffer, sender) => {
                let (state, response) = sender.tx(&buffer);
                core.crc.update_single(response);

                let response = Some(response);

                match state {
                    ArraySenderResult::Continue(sender) => Response {
                        response,
                        state: Self::Data(buffer, sender).into(),
                    },
                    ArraySenderResult::Done() => Response {
                        response,
                        state: State::SendResponseCRC,
                    },
                }
            }
        }
    }
}

impl TX11SDOResponse {
    pub fn new(data: ObjectBuffer, len: u8) -> Self {
        Self::Length(data, len)
    }
}
