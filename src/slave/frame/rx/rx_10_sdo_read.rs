use crate::{
    crc8::CRC,
    slave::frame::{
        core::Core,
        sdo::SDOAbort,
        state::State,
        tx::{TX11SDOResponse, TX1FSDOAbort, TXType},
        Receiver, Response,
    },
    Callbacks, ObjectBuffer,
};

use super::{OwnedStructReceiver, OwnedStructReceiverResult, RXType};

#[derive(Debug, PartialEq)]
pub struct RX10SDORead {
    receiver: OwnedStructReceiver<Ping>,
}

#[repr(align(1))]
#[derive(Debug, Default, PartialEq)]
struct Ping {
    universe: u8,
    address: u8,
    object_index: u16,
}

impl Receiver for RX10SDORead {
    fn rx(self, data: u8, core: &mut Core, callbacks: &mut Callbacks) -> Response {
        core.crc.update_single(data);

        match self.receiver.rx(data) {
            OwnedStructReceiverResult::Continue(receiver) => {
                Response::from_state(Self { receiver }.into())
            }
            OwnedStructReceiverResult::Done(v) => {
                let mut buf = ObjectBuffer::default();
                let callback_res = (callbacks.read_object)(v.object_index, &mut buf);

                match callback_res {
                    Ok(size) => State::WaitForCRC(Some(TXType::SDOResponse(TX11SDOResponse::new(
                        buf, size as u8,
                    ))))
                    .into(),
                    Err(e) => {
                        let abort_code = e.abort_code();

                        let abort = SDOAbort {
                            operation: 0x00,
                            index: v.object_index,
                            abort_code,
                        };

                        Response::from_state(TX1FSDOAbort::new(abort).into())
                    }
                }
            }
        }
    }
}

impl From<RX10SDORead> for State {
    fn from(value: RX10SDORead) -> Self {
        Self::HandleRX(RXType::SDORead(value))
    }
}

impl Default for RX10SDORead {
    fn default() -> Self {
        Self {
            receiver: OwnedStructReceiver::new(Ping::default()),
        }
    }
}
