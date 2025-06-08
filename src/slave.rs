use replace_with::replace_with_or_abort;

use crate::{
    command::Command,
    crc8::{CRC8Autosar, CRC},
    SINGLE_START_BYTE,
};

#[derive(Debug, Default)]
pub struct SlaveHandle {
    state: BusState,
    core: SlaveCore,
}

#[derive(Debug, Default)]
pub struct SlaveCore {
    in_sync: bool,
}

#[derive(PartialEq, Debug, Default)]
pub enum BusState {
    /// The idle bus state that waits for a start byte
    #[default]
    Idle,

    /// Wait for the command byte and parse it
    WaitForCommand(CRC8Autosar),
}

impl SlaveHandle {
    /// Handle an incoming byte from the bus endpoint
    /// # Arguments
    /// * `data` - The byte of data to be handled
    /// # Returns
    /// A possible byte to be sent back
    pub fn rx(&mut self, data: u8) -> Option<u8> {
        let mut response = None;

        replace_with_or_abort(&mut self.state, |s| {
            let (s, r) = s.rx(data, &mut self.core);
            response = r;
            s
        });

        response
    }

    /// Check if the bus has some data read to be sent
    /// # Returns
    /// A possible byte to be sent
    pub fn tx(&mut self) -> Option<u8> {
        let mut response = None;

        replace_with_or_abort(&mut self.state, |s| {
            let (s, r) = s.tx();
            response = r;
            s
        });

        response
    }
}

impl BusState {
    fn rx(self, data: u8, core: &mut SlaveCore) -> (Self, Option<u8>) {
        match self {
            //
            // In the idle state, we essentially wait for the start byte.
            // If we receive anything other than the start byte, we might
            // be out of sync with the bus and disable the `in_sync` flag
            //
            Self::Idle => (
                if data == SINGLE_START_BYTE {
                    Self::WaitForCommand(CRC8Autosar::new().update_single_move(data))
                } else {
                    Self::sync_lost(core)
                },
                None,
            ),

            //
            // Wait for the command byte to be received
            // and parse it. If that fails (unknown command)
            // we'll return to the Idle state
            //
            Self::WaitForCommand(crc) => match Command::from_u8(data) {
                Some(cmd) => {
                    let _crc = crc.update_single_move(data);
                    match cmd {
                        Command::NOP => (Self::Idle, None),
                        _ => panic!("Unimplemented command"),
                    }
                }
                None => (Self::sync_lost(core), None),
            },
        }
    }

    /// Change the core's sync flag to false and go back to Idle
    /// # Arguments
    /// * `core` - The core to drop out of sync
    /// # Returns
    /// The new state
    fn sync_lost(core: &mut SlaveCore) -> Self {
        core.in_sync = false;
        Self::Idle
    }

    fn tx(self) -> (Self, Option<u8>) {
        match self {
            x => (x, None),
        }
    }
}

#[cfg(test)]
mod tests {}
