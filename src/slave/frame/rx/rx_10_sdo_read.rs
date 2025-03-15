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

use super::{RXType, StructReceiver, StructReceiverResult};

#[derive(Debug, Default, PartialEq)]
pub struct RX10SDORead {
    structure: Structure,
    receiver: StructReceiver,
}

#[repr(align(1))]
#[derive(Debug, Default, PartialEq)]
struct Structure {
    universe: u8,
    address: u8,
    object_index: u16,
}

impl Receiver for RX10SDORead {
    fn rx(mut self, data: u8, core: &mut Core, callbacks: &mut Callbacks) -> Response {
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
                let mut buf = ObjectBuffer::default();
                let callback_res = (callbacks.read_object)(self.structure.object_index, &mut buf);

                match callback_res {
                    Ok(size) => State::WaitForCRC(Some(TXType::SDOResponse(TX11SDOResponse::new(
                        buf, size as u8,
                    ))))
                    .into(),
                    Err(e) => {
                        let abort_code = e.abort_code();

                        let abort = SDOAbort {
                            operation: 0x00,
                            index: self.structure.object_index,
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
