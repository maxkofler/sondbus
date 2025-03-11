mod tx_01_ping;
pub use tx_01_ping::TX01Ping;

mod tx_struct;
pub use tx_struct::*;

use crate::FrameType;

use super::{Receiver, Sender};

#[derive(Debug, PartialEq)]
pub enum TXType {
    Ping(TX01Ping),
}

impl TXType {
    pub fn to_frame_type(&self) -> FrameType {
        match self {
            Self::Ping(_) => FrameType::Ping,
        }
    }
}

impl Receiver for TXType {
    fn rx(self, data: u8, core: &mut super::core::Core) -> super::Response {
        match self {
            Self::Ping(v) => v.rx(data, core),
        }
    }
}

impl Sender for TXType {
    fn tx(self, core: &mut super::core::Core) -> super::Response {
        match self {
            Self::Ping(v) => v.tx(core),
        }
    }
}
