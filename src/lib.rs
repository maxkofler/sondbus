#![no_std]

pub mod frameactions;
pub mod frametype;

pub mod slave;
pub use slave::*;
pub mod crc8;

/// The byte that marks the start of a sondbus frame
pub const START_BYTE: u8 = 0x55;

/// The actions that follow a frame
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FrameAction {
    /// No action and response
    None,
}

/// A physical address for sondbus
#[derive(Debug, Default, Clone)]
pub struct PhysicalAddress {}

impl PhysicalAddress {
    /// Create a constant default address
    pub const fn const_default() -> Self {
        Self {}
    }
}

pub trait Bus {}
