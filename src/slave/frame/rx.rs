mod rx_00_sync;
pub use rx_00_sync::RX00Sync;

mod rx_01_ping;
pub use rx_01_ping::RX01Ping;

mod rx_10_sdo_read;
pub use rx_10_sdo_read::RX10SDORead;

mod rx_11_sdo_response;
pub use rx_11_sdo_response::RX11SDOResponse;

mod rx_1f_sdo_abort;
pub use rx_1f_sdo_abort::RX1FSDOAbort;

mod rx_struct;
pub use rx_struct::*;

use crate::{Callbacks, FrameType};

use super::{core::Core, state::State, Receiver, Response};

#[derive(Debug, PartialEq)]
pub enum RXType {
    Sync(RX00Sync),
    Ping(RX01Ping),
    SDORead(RX10SDORead),
    SDOResponse(RX11SDOResponse),
    SDOAbort(RX1FSDOAbort),
}

impl Receiver for RXType {
    fn rx(self, data: u8, core: &mut Core, callbacks: &mut Callbacks) -> Response {
        match self {
            Self::Sync(v) => v.rx(data, core, callbacks),
            Self::Ping(v) => v.rx(data, core, callbacks),
            Self::SDORead(v) => v.rx(data, core, callbacks),
            Self::SDOResponse(v) => v.rx(data, core, callbacks),
            Self::SDOAbort(v) => v.rx(data, core, callbacks),
        }
    }
}

impl From<FrameType> for RXType {
    fn from(value: FrameType) -> Self {
        value.to_rx_type()
    }
}

impl FrameType {
    fn to_rx_type(self) -> RXType {
        match self {
            FrameType::Sync => RXType::Sync(RX00Sync::default()),
            FrameType::Ping => RXType::Ping(RX01Ping::default()),
            FrameType::SDORead => RXType::SDORead(RX10SDORead::default()),
            FrameType::SDOResponse => RXType::SDOResponse(RX11SDOResponse::default()),
            FrameType::SDOAbort => RXType::SDOAbort(RX1FSDOAbort::default()),
        }
    }
}

impl From<RXType> for State {
    fn from(value: RXType) -> Self {
        State::HandleRX(value)
    }
}
