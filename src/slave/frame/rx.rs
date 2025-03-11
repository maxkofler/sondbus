mod rx_00_sync;
pub use rx_00_sync::RX00Sync;

mod rx_01_ping;
pub use rx_01_ping::RX01Ping;

use crate::FrameType;

use super::{state::State, Receiver, Response};

#[derive(Debug)]
pub enum RXType {
    Sync(RX00Sync),
    Ping(RX01Ping),
}

impl Receiver for RXType {
    fn rx(self, data: u8, core: &mut super::core::Core) -> Response {
        match self {
            Self::Sync(v) => v.rx(data, core),
            Self::Ping(v) => v.rx(data, core),
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
        }
    }
}

impl From<RXType> for State {
    fn from(value: RXType) -> Self {
        State::HandleRX(value)
    }
}
