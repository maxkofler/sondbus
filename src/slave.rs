//! Work with sondbus slaves
mod slavestate;
pub use slavestate::*;

use crate::{Bus, PhysicalAddress};

/// An instance of a sondbus slave
#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct Slave<const NUM_OBJECTS: u8> {
    /// The physical address for this device
    physical_address: PhysicalAddress,

    pub state: BusState,

    bus: SlaveBus,
}

/// The bus instance of the slave
#[derive(Clone, Default)]
struct SlaveBus {
    /// The logical network address of this slave
    pub address: Option<u8>,
}

impl Bus for SlaveBus {
    fn get_address(&self) -> Option<u8> {
        self.address
    }
}

impl SlaveBus {
    pub const fn const_default() -> Self {
        Self { address: None }
    }
}

impl<const NUM_OBJECTS: u8> Slave<NUM_OBJECTS> {
    pub const fn const_default() -> Self {
        Self {
            physical_address: PhysicalAddress::const_default(),
            state: BusState::const_default(),
            bus: SlaveBus::const_default(),
        }
    }

    pub fn new(physical_address: PhysicalAddress) -> Self {
        Self {
            physical_address,
            state: BusState::default(),
            bus: SlaveBus::default(),
        }
    }

    pub fn handle(mut self, data: u8) -> (Self, Option<u8>) {
        let (state, response) = self.state.handle(data, &mut self.bus);
        self.state = state;

        (self, response)
    }

    pub fn next(mut self) -> (Self, Option<u8>) {
        let (state, response) = self.state.next();
        self.state = state;

        (self, response)
    }

    /// Set the logical address of the bus
    /// # Arguments
    /// * `address` - The address to set
    pub fn set_address(mut self, address: Option<u8>) -> Self {
        self.bus.address = address;
        self
    }
}
