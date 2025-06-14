use crate::crc8::{CRC8Autosar, CRC};

#[derive(Debug)]
pub struct SlaveCore<const SCRATCHPAD_SIZE: usize> {
    in_sync: bool,
    scratchpad: [u8; SCRATCHPAD_SIZE],
    physical_address: [u8; 6],
    logical_address: u8,
    crc: CRC8Autosar,
}

impl<const S: usize> SlaveCore<S> {
    pub const fn default() -> Self {
        Self {
            in_sync: false,
            scratchpad: [0; S],
            physical_address: [0; 6],
            logical_address: 0,
            crc: CRC8Autosar::new_const(),
        }
    }

    pub fn set_logical_address(&mut self, address: u8) {
        self.logical_address = address;
    }

    pub fn logical_address(&self) -> u8 {
        self.logical_address
    }

    pub fn in_sync(&self) -> bool {
        self.in_sync
    }

    pub fn set_in_sync(&mut self, in_sync: bool) {
        self.in_sync = in_sync;
    }

    pub fn crc(&self) -> &CRC8Autosar {
        &self.crc
    }

    pub fn reset_crc(&mut self) {
        self.crc = CRC8Autosar::new();
    }

    pub fn update_crc_single(&mut self, data: u8) {
        self.crc.update_single(data);
    }

    pub fn scratchpad_mut(&mut self) -> &mut [u8; S] {
        &mut self.scratchpad
    }

    pub fn scratchpad(&self) -> &[u8; S] {
        &self.scratchpad
    }
}
