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

#[cfg(test)]
mod test {
    use crate::{
        crc8::{CRC8Autosar, CRC},
        START_BYTE,
    };

    use super::BusState;

    #[test]
    fn bus_waits_for_start() {
        let state = BusState::default();

        assert_eq!(
            state.clone().handle(0xFF).0,
            state,
            "Bus does not ignore non-start bytes"
        );
        assert_eq!(
            state.handle(START_BYTE).0,
            BusState::WaitForType,
            "Bus does not react to start byte"
        );
    }

    #[test]
    fn bus_handles_empty_frame() {
        let mut state = BusState::default();

        let data = [START_BYTE, 0, 0, 0, 0];

        for byte in data {
            state = state.handle(byte).0;
        }

        assert_eq!(
            state,
            BusState::WaitForStart,
            "Bus does not handle an empty frame"
        )
    }

    #[test]
    fn bus_crc_empty_frame() {
        let mut state = BusState::default();

        let data = [START_BYTE, 0, 0, 0];

        for byte in data {
            state = state.handle(byte).0;
        }

        if let BusState::WaitForCRC { ty: _, crc } = state {
            let real_crc = CRC8Autosar::new().update_move(&data);

            assert_eq!(real_crc.finalize(), crc.finalize());
        }
    }
}
