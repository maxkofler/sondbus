use replace_with::replace_with_or_abort;

use crate::{
    command::Command,
    crc8::{CRC8Autosar, CRC},
    SINGLE_START_BYTE, SYNC_MAGIC,
};

#[derive(Debug)]
pub struct SlaveHandle<const SCRATCHPAD_SIZE: usize> {
    state: BusState,
    core: SlaveCore<SCRATCHPAD_SIZE>,
}

#[derive(Debug)]
pub struct SlaveCore<const SCRATCHPAD_SIZE: usize> {
    in_sync: bool,
    scratchpad: [u8; SCRATCHPAD_SIZE],
}

#[derive(PartialEq, Debug, Default)]
pub enum BusState {
    /// The idle bus state that waits for a start byte
    #[default]
    Idle,

    /// Wait for the command byte and parse it
    WaitForCommand(CRC8Autosar),

    /// Process the sync command
    Sync(CRC8Autosar, usize),

    /// Wait for the high byte of the incoming offset of a write command
    WriteOffsetH { crc: CRC8Autosar, respond: bool },

    /// Wait for the low byte of the incoming offset of a write command
    WriteOffsetL {
        crc: CRC8Autosar,
        respond: bool,
        offset: u8,
    },

    /// Wait for the incoming length of a write command
    WriteLength {
        crc: CRC8Autosar,
        respond: bool,
        offset: u16,
    },

    /// Wait and process the incoming data of a write command
    WriteData {
        crc: CRC8Autosar,
        respond: bool,
        offset: u16,
        length: u8,
        written: u8,
    },

    /// Wait for the final CRC checksum of the frame
    WaitForCRC(u8, BusAction),
}

/// The possible actions that follow a frame
#[derive(PartialEq, Debug, Default)]
pub enum BusAction {
    /// Do nothing
    #[default]
    None,

    /// Set the `in_sync` flag
    SetInSync(bool),

    /// Respond with the CRC
    WriteAndRespondCRC(CRC8Autosar),
}

impl<const SCRATCHPAD_SIZE: usize> SlaveHandle<SCRATCHPAD_SIZE> {
    /// A `const` variant of `default()` that allows const
    /// construction of a slave handle
    pub const fn default() -> Self {
        Self {
            state: BusState::Idle,
            core: SlaveCore {
                in_sync: false,
                scratchpad: [0u8; SCRATCHPAD_SIZE],
            },
        }
    }

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

    /// Returns the sync state of the slave instance
    pub fn in_sync(&self) -> bool {
        self.core.in_sync
    }
}

impl BusState {
    fn rx<const SCRATCHPAD_SIZE: usize>(
        self,
        data: u8,
        core: &mut SlaveCore<SCRATCHPAD_SIZE>,
    ) -> (Self, Option<u8>) {
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
                    let crc = crc.update_single_move(data);

                    // We only handle a command if we are in sync or
                    // it is a `SYN` command. Otherwise we'll fall out
                    // of sync again just to be sure
                    if core.in_sync || cmd == Command::SYN {
                        match cmd {
                            Command::NOP => {
                                (Self::WaitForCRC(crc.finalize(), BusAction::None), None)
                            }
                            Command::SYN => (Self::Sync(crc, 0), None),
                            Command::BWR => (
                                Self::WriteOffsetH {
                                    crc,
                                    respond: false,
                                },
                                None,
                            ),
                            _ => panic!("Unimplemented command"),
                        }
                    } else {
                        (Self::sync_lost(core), None)
                    }
                }
                None => (Self::sync_lost(core), None),
            },

            //
            // Wait for the bytes of the sync sequence to come
            // in and check their correctness. If ok, move to
            // the next and finally to the CRC for validation.
            // If we receive any wrong byte, immediately loose
            // sync.
            //
            Self::Sync(crc, pos) => {
                let crc = crc.update_single_move(data);

                let state = if SYNC_MAGIC[pos] == data {
                    if pos >= 14 {
                        Self::WaitForCRC(crc.finalize(), BusAction::SetInSync(true))
                    } else {
                        Self::Sync(crc, pos + 1)
                    }
                } else {
                    Self::sync_lost(core)
                };

                (state, None)
            }

            //
            // Wait for the final CRC checksum to confirm
            // correct reception of the command
            //
            Self::WaitForCRC(crc, action) => {
                if crc == data {
                    match action {
                        BusAction::None => (Self::Idle, None),
                        BusAction::SetInSync(sync) => {
                            core.in_sync = sync;
                            (Self::Idle, None)
                        }
                        BusAction::WriteAndRespondCRC(crc) => {
                            // TODO: Write
                            (Self::Idle, Some(crc.update_single_move(data).finalize()))
                        }
                    }
                } else {
                    (Self::sync_lost(core), None)
                }
            }

            //
            // Wait for the incoming high byte of the offset
            // to write at using a write command
            //
            Self::WriteOffsetH { crc, respond } => (
                Self::WriteOffsetL {
                    crc: crc.update_single_move(data),
                    respond,
                    offset: data,
                },
                None,
            ),

            //
            // Wait for the incoming low byte of the offset
            // to write at using a write command
            //
            Self::WriteOffsetL {
                crc,
                respond,
                offset,
            } => (
                Self::WriteLength {
                    crc: crc.update_single_move(data),
                    respond,
                    offset: (offset as u16) << 8 | data as u16,
                },
                None,
            ),

            //
            // Wait for the incoming length of a write command
            //
            Self::WriteLength {
                crc,
                respond,
                offset,
            } => {
                // Check if we're able to fit the data into the scratchpad,
                // otherwise, we'll loose sync as something fishy might go on
                if data > SCRATCHPAD_SIZE as u8 {
                    (Self::sync_lost(core), None)
                } else {
                    (
                        Self::WriteData {
                            crc: crc.update_single_move(data),
                            respond,
                            offset,
                            length: data,
                            written: 0,
                        },
                        None,
                    )
                }
            }

            //
            // Wait and note the incoming data of a write command
            //
            Self::WriteData {
                crc,
                respond,
                offset,
                length,
                written,
            } => {
                core.scratchpad[written as usize] = data;

                let written = written + 1;
                let crc = crc.update_single_move(data);

                if written >= length {
                    if respond {
                        (
                            Self::WaitForCRC(crc.finalize(), BusAction::WriteAndRespondCRC(crc)),
                            None,
                        )
                    } else {
                        (Self::WaitForCRC(crc.finalize(), BusAction::None), None)
                    }
                } else {
                    (
                        Self::WriteData {
                            crc,
                            respond,
                            offset,
                            length,
                            written,
                        },
                        None,
                    )
                }
            }
        }
    }

    /// Change the core's sync flag to false and go back to Idle
    /// # Arguments
    /// * `core` - The core to drop out of sync
    /// # Returns
    /// The new state
    fn sync_lost<const SCRATCHPAD_SIZE: usize>(core: &mut SlaveCore<SCRATCHPAD_SIZE>) -> Self {
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
mod tests {
    use crate::{
        command::Command,
        crc8::{CRC8Autosar, CRC},
        slave::{BusAction, BusState, SlaveHandle},
        SINGLE_START_BYTE, SYNC_MAGIC,
    };

    /// Check that the bus correctly transitions from `Idle` to `WaitForCommand`
    /// when receiving a single-command start byte
    #[test]
    fn idle_to_cmd() {
        let mut slave = SlaveHandle::<0>::default();

        // This transition does NEVER yield a response
        assert_eq!(
            slave.rx(SINGLE_START_BYTE),
            None,
            "Idle -> Command incorrectly responds"
        );

        // Check that the internal state moved to the correct next state
        assert_eq!(
            slave.state,
            BusState::WaitForCommand(CRC8Autosar::new().update_single_move(SINGLE_START_BYTE)),
            "Idle -> Command does not happen correctly"
        )
    }

    /// Test that the `in_sync` flag is kept as-is for a
    /// correct transition from `Idle` to `WaitForCommand`
    /// and that it is reset if an invalid byte is received
    #[test]
    fn idle_to_cmd_sync() {
        // Check that in_sync stays false for correct transitions
        let mut slave = SlaveHandle::<0>::default();
        assert_eq!(
            slave.core.in_sync, false,
            "in_sync is not false for new instance"
        );
        slave.rx(SINGLE_START_BYTE);
        assert!(!slave.core.in_sync, "in_sync changed unexpectedly");

        // Check that in_sync stays true for correct transitions
        let mut slave = SlaveHandle::<0>::default();
        slave.core.in_sync = true;
        slave.rx(SINGLE_START_BYTE);
        assert!(slave.core.in_sync, "in_sync changed unexpectedly");

        // Check that in_sync is stays false for incorrect bytes
        let mut slave = SlaveHandle::<0>::default();
        slave.rx(SINGLE_START_BYTE + 0x34);
        assert!(!slave.core.in_sync, "in_sync is set for false starts");

        // Check that in_sync is set from true to false for incorrect bytes
        let mut slave = SlaveHandle::<0>::default();
        slave.core.in_sync = true;
        slave.rx(SINGLE_START_BYTE + 0x34);
        assert!(
            !slave.core.in_sync,
            "in_sync is not de-asserted for false starts"
        );
    }

    /// Test the `NOP` command
    #[test]
    fn cmd_nop() {
        let mut slave = SlaveHandle::<0>::default();
        slave.core.in_sync = true;

        let mut crc = CRC8Autosar::new();

        assert_eq!(slave.rx(SINGLE_START_BYTE), None);
        crc.update_single(SINGLE_START_BYTE);
        assert_eq!(slave.state, BusState::WaitForCommand(crc.clone()));

        let cmd = Command::NOP.u8();
        assert_eq!(slave.rx(cmd), None);
        crc.update_single(cmd);
        assert_eq!(
            slave.state,
            BusState::WaitForCRC(crc.finalize(), BusAction::None)
        );
    }

    /// Test the `SYNC` command
    #[test]
    fn cmd_sync() {
        let mut slave = SlaveHandle::<0>::default();
        let mut crc = CRC8Autosar::new();

        crc.update(&[SINGLE_START_BYTE, Command::SYN.u8()]);

        assert_eq!(slave.rx(SINGLE_START_BYTE), None);
        assert_eq!(slave.rx(Command::SYN.u8()), None);

        for data in SYNC_MAGIC {
            assert_eq!(slave.rx(data), None);
            crc.update_single(data);
        }

        assert_eq!(
            slave.state,
            BusState::WaitForCRC(crc.finalize(), BusAction::SetInSync(true))
        );
        assert_eq!(slave.rx(crc.finalize()), None);
        assert!(slave.core.in_sync)
    }
}
