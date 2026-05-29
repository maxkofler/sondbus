//! The transceiver implements the lowest layer of the sondbus communication protocol
//! and handles synchronization of the communication and memory access.

mod state;

use super::super::crc8::{CRC8Autosar, CRC};

use crate::model::command::{Command, ManagementCommand};
use state::State;

use crate::test_log;

/// Consequences of commands that are executed if a
/// command is finished with the right CRC
#[derive(PartialEq, Debug)]
enum Consequence {
    /// Nothing, return back to idle
    None,

    GainSync,
}

type StateFunction = fn(&mut Transceiver, rx: Option<u8>) -> Option<u8>;

/// The possible actions that can be requested
/// when the callback is called
pub enum CallbackAction<'a> {
    /// Write the contents of `data` to memory at `offset`
    WriteMemory { offset: u16, data: &'a [u8] },

    ///  Read from memory memory at `offset` to `data`
    ReadMemory { offset: u16, data: &'a mut [u8] },
}

/// A type alias for the callback
pub type Callback = for<'a> fn(CallbackAction<'a>) -> Result<(), ()>;

/// Represents a transceiver in the sondbus model.
///
/// The transceiver implements the lowest layer of the sondbus communication protocol
/// and handles synchronization of the communication and slave memory access.
pub struct Transceiver<'a> {
    /// The current state the transceiver is in
    state: State,

    /// The crc of a message up to the current state for CRC
    /// calculations
    crc: CRC8Autosar,

    /// The current command being received / processed
    cur_cmd: Command,

    /// Whether or not the transceiver is in sync
    in_sync: bool,

    /// The sequence number of the last successfully processed message
    sequence_no: u8,

    /// The scratchpad memory used to temporarily store incoming data
    /// before it is committed using the callback
    scratchpad: &'a mut [u8],

    /// The current position in a buffer
    pos: u8,

    consequence: Consequence,
}

impl<'a> Transceiver<'a> {
    /// Creates a new transceiver
    /// # Arguments
    /// * `scratchpad` - The scratchpad memory to operate on
    /// * `physical_address` - The unique physical address of the transceiver
    /// * `callback` - A callback function for the transceiver to call into the application
    pub const fn new(
        scratchpad: &'a mut [u8],
        physical_address: [u8; 6],
        callback: Callback,
    ) -> Self {
        Self {
            state: State::Idle,
            crc: CRC8Autosar::new_const(),
            cur_cmd: Command::Management(ManagementCommand::Nop),
            in_sync: false,
            sequence_no: 0,
            scratchpad,
            pos: 0,
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

    fn update_crc(&mut self, v: u8) {
        self.crc.update_single(v)
    }
}

#[cfg(test)]
impl<'a> Transceiver<'a> {
    pub fn t_handle_no_response(&mut self, rx: u8) {
        let old_state = self.state.clone();
        let res = self.handle(Some(rx));
        assert!(
            res.is_none(),
            "Handling {rx:x} in state {old_state:?} responded when it should not"
        );
    }

    pub fn t_handle_crc(&mut self) {
        self.t_handle_no_response(self.crc.finalize());
    }
}

#[cfg(test)]
mod test;
