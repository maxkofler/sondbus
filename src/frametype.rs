use crate::{Bus, FrameAction};

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
            0x10 => Some(Self::CyclicRequest(CyclicRequest::default())),
            _ => None,
        }
    }

    /// Pass in the target address of the frame data that is to be coming in
    /// # Arguments
    /// * `addr` - The address to set
    pub fn address(self, addr: u8) -> Self {
        self
    }

    /// Pass in the length of the data to be received
    ///
    /// This function is called after `address()`
    /// # Arguments
    /// * `length` - The length the incoming data is to be expected
    pub fn length(self, length: u8) -> Self {
        self
    }

    /// Handle a incoming data byte from the data link layer
    ///
    /// This function is called after `length()`
    /// # Arguments
    /// * `data` - The data byte to handle
    pub fn handle(self, data: u8) -> Self {
        match self {
            Self::CyclicRequest(request) => Self::CyclicRequest(request),
        }
    }

    /// Commit this frame's information to the bus
    ///
    /// This function is called after `handle()` and is the last function
    /// # Arguments
    /// * `bus` - The bus to commit the changes to / derive the action from
    /// # Returns
    /// An action that is derived from this frame
    pub fn commit(self, bus: &mut dyn Bus) -> FrameAction {
        FrameAction::None
    }
}

/// A cyclic request frame requesting cyclic data
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CyclicRequest {}
