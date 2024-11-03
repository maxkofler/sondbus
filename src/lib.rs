#![no_std]

pub mod frameaction;
pub mod frametype;

pub mod slave;
use frameaction::FrameAction;
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

/// Trait for functions that can handle frame data
pub trait FrameDataHandler: Sized {
    /// Pass in the target address of the frame data that is to be coming in
    /// # Arguments
    /// * `addr` - The address to set
    fn address(self, _addr: u8) -> Self {
        self
    }

    /// Pass in the length of the data to be received
    ///
    /// This function is called after `address()`
    /// # Arguments
    /// * `length` - The length the incoming data is to be expected
    fn length(self, _length: u8) -> Self {
        self
    }

    /// Handle a incoming data byte from the data link layer
    ///
    /// This function is called after `length()`
    /// # Arguments
    /// * `data` - The data byte to handle
    fn handle(self, _data: u8) -> Self {
        self
    }

    /// Commit this frame's information to the bus
    ///
    /// This function is called after `handle()` and is the last function
    /// # Arguments
    /// * `bus` - The bus to commit the changes to / derive the action from
    /// # Returns
    /// An action that is derived from this frame
    fn commit(self, _bus: &mut dyn Bus) -> FrameAction;
}
