//! The transceiver implements the lowest layer of the sondbus communication protocol
//! and handles synchronization of the communication and memory access.

mod command;
mod state;

#[cfg(test)]
mod test;

use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::transceiver::state::State,
    test_log,
};
use command::Command;

type StateFunction = fn(&mut Transceiver, rx: Option<u8>) -> Option<u8>;

/// Consequences of commands that are executed if a
/// command is finished with the right CRC
#[derive(PartialEq, Debug)]
enum Consequence {
    /// Nothing, return back to idle
    None,

    GainSync,

    /// Write the contents of the scratchpad to the
    /// slave's memory area
    WriteScratchpad,
}

/// Represents a transceiver in the sondbus model.
///
/// The transceiver implements the lowest layer of the sondbus communication protocol
/// and handles synchronization of the communication and slave memory access.
pub struct Transceiver<'a> {
    state: State,
    crc: CRC8Autosar,
    cur_cmd: Command,

    in_sync: bool,
    sequence_no: u8,

    pos: u16,

    mem_cmd_addr: [u8; 6],
    mem_cmd_offset: u16,
    mem_cmd_size: u16,

    physical_address: [u8; 6],
    logical_address: [u8; 2],

    scratchpad: &'a mut [u8],
    consequence: Consequence,
}

impl<'a> Transceiver<'a> {
    /// Creates a new transceiver
    /// # Arguments
    /// * `scratchpad` - The scratchpad memory to operate on
    pub const fn new(scratchpad: &'a mut [u8], physical_address: [u8; 6]) -> Self {
        Self {
            state: State::WaitForStart,
            crc: CRC8Autosar::new_const(),
            cur_cmd: Command::new(0),

            in_sync: false,
            sequence_no: 0,

            pos: 0,
            mem_cmd_addr: [0u8; 6],
            mem_cmd_offset: 0,
            mem_cmd_size: 0,

            physical_address,
            logical_address: [0u8; 2],

            scratchpad,

            consequence: Consequence::None,
        }
    }

    /// Returns whether the bus is in sync or not
    pub fn in_sync(&self) -> bool {
        self.in_sync
    }

    /// Sets the internal `in_sync` flag false, effectively
    /// taking the transceiver offline until the next `Sync`
    /// command comes around from the master
    pub fn loose_sync(&mut self) {
        test_log!("Lost sync!");
        self.in_sync = false;
    }

    /// Process some event in the state machine of the transceiver.
    /// # Arguments
    /// * `rx` - An incoming byte from the physical layer
    /// # Returns
    /// A byte to be sent via the physical layer, if any
    #[allow(clippy::let_and_return)] // This is to remove a warning around test_log!()
    pub fn handle(&mut self, rx: Option<u8>) -> Option<u8> {
        #[cfg(test)]
        let old_state = self.state.clone();

        let res = state::handle(self, rx);

        test_log!("Transitioned from {:?} to {:?}", old_state, self.state);

        res
    }

    /// Returns whether the data that is coming up to this point
    /// is targeted to us or another slave
    fn is_targeted(&self) -> bool {
        if !self.cur_cmd.is_mem_cmd() {
            return false;
        }

        match self.cur_cmd.mem_slave_address_len() {
            0 => true,
            2 => self.mem_cmd_addr[0..2] == self.logical_address,
            6 => self.mem_cmd_addr == self.physical_address,
            _ => false,
        }
    }

    fn update_crc(&mut self, v: u8) {
        self.crc.update_single(v)
    }
}
