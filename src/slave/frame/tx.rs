mod tx_01_ping;
pub use tx_01_ping::TX01Ping;

mod tx_11_sdo_response;
pub use tx_11_sdo_response::TX11SDOResponse;

mod tx_1f_sdo_abort;
pub use tx_1f_sdo_abort::TX1FSDOAbort;

mod tx_struct;
pub use tx_struct::*;

mod tx_array;
pub use tx_array::*;

use crate::{Callbacks, FrameType};

use super::{core::Core, Receiver, Response, Sender};

#[derive(Debug, PartialEq)]
pub enum TXType {
    Ping(TX01Ping),
    SDOResponse(TX11SDOResponse),
    SDOAbort(TX1FSDOAbort),
}

impl TXType {
    pub fn to_frame_type(&self) -> FrameType {
        match self {
            Self::Ping(_) => FrameType::Ping,
            Self::SDOResponse(_) => FrameType::SDOResponse,
            Self::SDOAbort(_) => FrameType::SDOAbort,
        }
    }
}

impl Receiver for TXType {
    fn rx(self, data: u8, core: &mut Core, callbacks: &mut Callbacks) -> Response {
        match self {
            Self::Ping(v) => v.rx(data, core, callbacks),
            Self::SDOResponse(v) => v.rx(data, core, callbacks),
            Self::SDOAbort(v) => v.rx(data, core, callbacks),
        }
    }
}

impl Sender for TXType {
    fn tx(self, core: &mut Core, callbacks: &mut Callbacks) -> Response {
        match self {
            Self::Ping(v) => v.tx(core, callbacks),
            Self::SDOResponse(v) => v.tx(core, callbacks),
            Self::SDOAbort(v) => v.tx(core, callbacks),
        }
    }
}
