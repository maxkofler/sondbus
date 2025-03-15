#![no_std]

pub mod slave;
pub use slave::*;
pub mod crc8;
pub mod ringbuf;

/// The byte that marks the start of a sondbus frame
pub const START_BYTE: u8 = 0x55;

/// The byte sequence of the `SYNC` frame type (0x00)
pub const SYNC_SEQUENCE: [u8; 15] = [
    0x1F, 0x2E, 0x3D, 0x4C, 0x5B, 0x6A, 0x79, 0x88, 0x97, 0xA6, 0xB5, 0xC4, 0xD3, 0xE2, 0xF1,
];

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

    /*
    /// Commit this frame's information to the bus
    ///
    /// This function is called after `handle()` and is the last function
    /// # Arguments
    /// * `bus` - The bus to commit the changes to / derive the action from
    /// # Returns
    /// An action that is derived from this frame
    fn commit(self, _bus: &mut dyn Bus) -> FrameAction;
    */
}

#[repr(u8)]
pub enum FrameType {
    Sync = 0x00,
    Ping = 0x01,
    SDORead = 0x10,
    SDOResponse = 0x11,
    SDOAbort = 0x1F,
}

impl FrameType {
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Self::Sync),
            0x01 => Some(Self::Ping),
            0x10 => Some(Self::SDORead),
            0x11 => Some(Self::SDORead),
            _ => None,
        }
    }
}
