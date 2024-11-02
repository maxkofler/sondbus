//! Work with sondbus slaves
mod slavestate;
pub use slavestate::*;

use crate::{Bus, PhysicalAddress};

/// An instance of a sondbus slave
#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct Slave<const NUM_OBJECTS: u8> {
    /// The logical network address of this slave
    address: Option<u8>,

    /// The physical address for this device
    physical_address: PhysicalAddress,

    state: BusState,

    bus: SlaveBus,
}

#[derive(Clone, Default)]
struct SlaveBus {}

impl Bus for SlaveBus {}

impl SlaveBus {
    pub const fn const_default() -> Self {
        Self {}
    }
}

impl<const NUM_OBJECTS: u8> Slave<NUM_OBJECTS> {
    pub const fn const_default() -> Self {
        Self {
            address: None,
            physical_address: PhysicalAddress::const_default(),
            state: BusState::const_default(),
            bus: SlaveBus::const_default(),
        }
    }

    pub fn new(physical_address: PhysicalAddress) -> Self {
        Self {
            address: None,
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
}
