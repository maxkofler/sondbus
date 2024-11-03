#![no_std]

pub mod frameaction;
pub mod frametype;

pub mod slave;
pub use slave::*;
pub mod crc8;

/// The byte that marks the start of a sondbus frame
pub const START_BYTE: u8 = 0x55;

/// A physical address for sondbus
#[derive(Debug, Default, Clone)]
pub struct PhysicalAddress {}

impl PhysicalAddress {
    /// Create a constant default address
    pub const fn const_default() -> Self {
        Self {}
    }
}

pub trait Bus {
    /// Returns the logical address of this bus participant
    fn get_address(&self) -> Option<u8> {
        None
    }
}
