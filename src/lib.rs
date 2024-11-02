#![no_std]

pub mod frameactions;
pub mod frametypes;

pub mod slave;
pub use slave::*;
pub mod crc8;

use frametypes::CyclicRequest;

/// The byte that marks the start of a sondbus frame
pub const START_BYTE: u8 = 0x55;

/// The various frame types within sondbus
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum FrameType {
    /// A cyclic request frame `0x10`
    CyclicRequest(CyclicRequest) = 0x10,
}

impl FrameType {
    /// Derive a frame type from a `u8`
    pub fn from_u8(data: u8) -> Option<Self> {
        match data {
            0x10 => Some(Self::CyclicRequest(CyclicRequest::default())),
            _ => None,
        }
    }

    pub fn process(self, _data: u8, _addr: u8) -> Self {
        self
    }

    pub fn commit(self) -> FrameAction {
        FrameAction::None
    }
}

/// The actions that follow a frame
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FrameAction {
    /// No action and response
    None,
}

/// A physical address for sondbus
#[derive(Debug, Default)]
pub struct PhysicalAddress {}

impl PhysicalAddress {
    /// Create a constant default address
    pub const fn const_default() -> Self {
        Self {}
    }
}
