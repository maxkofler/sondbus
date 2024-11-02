//! Work with sondbus slaves
mod slavestate;
pub use slavestate::*;

use crate::PhysicalAddress;

/// An instance of a sondbus slave
#[allow(dead_code)]
pub struct Slave<const NUM_OBJECTS: u8> {
    /// The logical network address of this slave
    address: Option<u8>,

    /// The physical address for this device
    physical_address: PhysicalAddress,

    bus_state: BusState,
}

impl<const NUM_OBJECTS: u8> Slave<NUM_OBJECTS> {
    pub const fn const_default() -> Self {
        Self {
            address: None,
            physical_address: PhysicalAddress::const_default(),
            bus_state: BusState::const_default(),
        }
    }

    pub fn new(physical_address: PhysicalAddress) -> Self {
        Self {
            address: None,
            physical_address,
            bus_state: BusState::default(),
        }
    }

    pub fn handle(mut self, data: u8) -> (Self, Option<u8>) {
        let (state, response) = self.bus_state.handle(data);
        self.bus_state = state;

        (self, response)
    }
}
