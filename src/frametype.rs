//! The various types of frames available within sondbus
use crate::{frameaction::FrameAction, Bus, FrameDataHandler};

mod cyclic_request;
pub use cyclic_request::*;

/// The various frame types within sondbus
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum FrameType {
    /// A cyclic request frame `0x10`
    CyclicRequest(CyclicRequest) = 0x10,
}

impl FrameType {
    /// Derive a frame type from a `u8`
    /// # Arguments
    /// * `data` - The byte to derive the frame type from
    /// * `bus` - The bus to operate on
    pub fn from_u8(data: u8, bus: &dyn Bus) -> Option<Self> {
        match data {
            0x10 => Some(Self::CyclicRequest(CyclicRequest::new(bus.get_address()))),
            _ => None,
        }
    }
}

impl FrameDataHandler for FrameType {
    fn address(self, _addr: u8) -> Self {
        match self {
            Self::CyclicRequest(request) => Self::CyclicRequest(request),
        }
    }

    fn handle(self, data: u8) -> Self {
        match self {
            Self::CyclicRequest(request) => Self::CyclicRequest(request.handle(data)),
        }
    }

    fn commit(self, bus: &mut dyn Bus) -> FrameAction {
        match self {
            Self::CyclicRequest(request) => request.commit(bus),
        }
    }
}
