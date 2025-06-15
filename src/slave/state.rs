use crate::{
    command::Command,
    crc8::CRC,
    slave::{BusAction, CallbackAction, SlaveCore},
    SINGLE_START_BYTE, SYNC_MAGIC,
};

#[derive(Clone, PartialEq, Debug, Default)]
pub enum SlaveState {
    /// The idle bus state that waits for a start byte
    #[default]
    Idle,

    /// Wait for the command byte and parse it
    WaitForCommand,

    /// Process the sync command
    Sync(usize),

    /// Wait for the logical address of a write command
    WriteLogicalAddress,

    /// Wait for the logical address of a read command
    ReadLogicalAddress,

    /// Wait for the incoming offset of a write command
    WriteOffset { accept: bool, respond: bool },

    /// Wait for the incoming offset of a read command
    ReadOffset { accept: bool },

    /// Wait for the incoming length of a write command
    WriteLength {
        accept: bool,
        respond: bool,
        offset: u8,
    },

    /// Wait for the incoming length of a read command
    ReadLength { accept: bool, offset: u8 },

    /// Wait and process the incoming data of a write command
    WriteData {
        accept: bool,
        respond: bool,
        offset: u8,
        length: u8,
        written: u8,
    },

    /// Send the read data
    SendReadData { length: u8, sent: u8 },

    /// Wait and process the read data of another slave
    ReceiveReadData { length: u8, sent: u8 },

    /// Wait for the final CRC checksum of the frame
    WaitForCRC(u8, BusAction),

    /// Send the crc upon the next possibility
    SendCRC(u8),
}

impl SlaveState {
    pub fn rx<const SCRATCHPAD_SIZE: usize, F: FnMut(CallbackAction) -> bool>(
        self,
        data: u8,
        core: &mut SlaveCore<SCRATCHPAD_SIZE>,
        mut callback: F,
    ) -> (Self, Option<u8>) {
        core.update_crc_single(data);

        match self {
            //
            // In the idle state, we essentially wait for the start byte.
            // If we receive anything other than the start byte, we might
            // be out of sync with the bus and disable the `in_sync` flag
            //
            Self::Idle => (
                if data == SINGLE_START_BYTE {
                    core.reset_crc();
                    core.update_crc_single(data);
                    Self::WaitForCommand
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
            Self::WaitForCommand => match Command::from_u8(data) {
                Some(cmd) => {
                    // We only handle a command if we are in sync or
                    // it is a `SYN` command. Otherwise we'll fall out
                    // of sync again just to be sure
                    if core.in_sync() || cmd == Command::SYN {
                        match cmd {
                            Command::NOP => (
                                Self::WaitForCRC(core.crc().finalize(), BusAction::None),
                                None,
                            ),
                            Command::SYN => (Self::Sync(0), None),
                            Command::BWR => (
                                Self::WriteOffset {
                                    accept: true,
                                    respond: false,
                                },
                                None,
                            ),
                            Command::LWR => (Self::WriteLogicalAddress, None),
                            Command::LRD => (Self::ReadLogicalAddress, None),
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
            Self::Sync(pos) => {
                let state = if SYNC_MAGIC[pos] == data {
                    if pos >= 14 {
                        Self::WaitForCRC(core.crc().finalize(), BusAction::SetInSync(true))
                    } else {
                        Self::Sync(pos + 1)
                    }
                } else {
                    Self::sync_lost(core)
                };

                (state, None)
            }

            //
            // Wait for the logical address of a write command
            //
            Self::WriteLogicalAddress => (
                Self::WriteOffset {
                    accept: data == core.logical_address(),
                    respond: true,
                },
                None,
            ),

            //
            // Wait for the logical address of a read command
            //
            Self::ReadLogicalAddress => (
                Self::ReadOffset {
                    accept: data == core.logical_address(),
                },
                None,
            ),

            //
            // Wait for the incoming byte of the offset
            // to write at using a write command
            //
            Self::WriteOffset { accept, respond } => (
                Self::WriteLength {
                    accept,
                    respond,
                    offset: data,
                },
                None,
            ),

            //
            // Wait for the incoming byte of the offset
            // to read at using a read command
            //
            Self::ReadOffset { accept } => (
                Self::ReadLength {
                    accept,
                    offset: data,
                },
                None,
            ),

            //
            // Wait for the incoming length of a write command
            //
            Self::WriteLength {
                accept,
                respond,
                offset,
            } => {
                // Check if we're able to fit the data into the scratchpad,
                // otherwise, we'll loose sync as something fishy might go on
                if data > SCRATCHPAD_SIZE as u8 {
                    (Self::sync_lost(core), None)
                } else if data == 0 {
                    let action = if respond {
                        if accept {
                            BusAction::RespondCRC
                        } else {
                            BusAction::WaitForSecondCRC
                        }
                    } else {
                        BusAction::None
                    };
                    (Self::WaitForCRC(core.crc().finalize(), action), None)
                } else {
                    (
                        Self::WriteData {
                            accept,
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
            // Wait for the incoming length of a read command
            //
            Self::ReadLength { accept, offset } => (
                {
                    let action = if accept {
                        if data == 0 {
                            BusAction::RespondCRC
                        } else {
                            BusAction::ReadAndRespond(offset as u16, data)
                        }
                    } else if data == 0 {
                        BusAction::WaitForSecondCRC
                    } else {
                        BusAction::WaitForOtherRead(data)
                    };

                    Self::WaitForCRC(core.crc().finalize(), action)
                },
                None,
            ),

            //
            // Wait and note the incoming data of a write command
            //
            Self::WriteData {
                accept,
                respond,
                offset,
                length,
                written,
            } => {
                core.scratchpad_mut()[written as usize] = data;

                let written = written + 1;

                if written >= length {
                    // If we are done with the transmission, we'll take
                    // the action and wait for the CRC
                    let action = if respond {
                        if accept {
                            BusAction::WriteAndRespondCRC(offset as u16, length)
                        } else {
                            BusAction::WaitForSecondCRC
                        }
                    } else {
                        BusAction::WriteAndIdle(offset as u16, length)
                    };

                    (Self::WaitForCRC(core.crc().finalize(), action), None)
                } else {
                    (
                        Self::WriteData {
                            accept,
                            respond,
                            offset,
                            length,
                            written,
                        },
                        None,
                    )
                }
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
                            core.set_in_sync(sync);
                            (Self::Idle, None)
                        }
                        // Write the data out to the memory area and go back to the
                        // idle state
                        BusAction::WriteAndIdle(offset, len) => {
                            callback(CallbackAction::Write(
                                offset,
                                &core.scratchpad()[0..len as usize],
                            ));

                            (Self::Idle, None)
                        }
                        // Write the data out to the memory area and respond
                        // with a CRC checksum
                        BusAction::WriteAndRespondCRC(offset, len) => {
                            let res = callback(CallbackAction::Write(
                                offset,
                                &core.scratchpad()[0..len as usize],
                            ));
                            if res {
                                (Self::Idle, Some(core.crc().finalize()))
                            } else {
                                (Self::sync_lost(core), None)
                            }
                        }

                        BusAction::RespondCRC => (Self::Idle, Some(core.crc().finalize())),

                        BusAction::WaitForSecondCRC => (
                            Self::WaitForCRC(core.crc().finalize(), BusAction::None),
                            None,
                        ),

                        BusAction::ReadAndRespond(offset, len) => {
                            callback(CallbackAction::Read(
                                offset,
                                &mut core.scratchpad_mut()[0..len as usize],
                            ));

                            core.update_crc_single(core.scratchpad()[0]);

                            let state = if len > 1 {
                                Self::SendReadData {
                                    length: len,
                                    sent: 1,
                                }
                            } else {
                                Self::SendCRC(core.crc().finalize())
                            };

                            (state, Some(core.scratchpad()[0]))
                        }

                        BusAction::WaitForOtherRead(length) => {
                            (Self::ReceiveReadData { length, sent: 0 }, None)
                        }
                    }
                } else {
                    (Self::sync_lost(core), None)
                }
            }

            //
            // We should never receive anything in the send read data
            // state. If we do, we're out of sync
            //
            Self::SendReadData { length: _, sent: _ } => (Self::sync_lost(core), None),

            //
            // Receive and wait for the receive data of another
            // slave to go through
            //
            Self::ReceiveReadData { length, sent } => {
                if sent >= length - 1 {
                    (
                        Self::WaitForCRC(core.crc().finalize(), BusAction::None),
                        None,
                    )
                } else {
                    (
                        Self::ReceiveReadData {
                            length,
                            sent: sent + 1,
                        },
                        None,
                    )
                }
            }

            //
            // We should never receive when we're sending the CRC
            // ourselves. If we do, we're out of sync
            //
            Self::SendCRC(_crc) => (Self::sync_lost(core), None),
        }
    }

    /// Change the core's sync flag to false and go back to Idle
    /// # Arguments
    /// * `core` - The core to drop out of sync
    /// # Returns
    /// The new state
    fn sync_lost<const SCRATCHPAD_SIZE: usize>(core: &mut SlaveCore<SCRATCHPAD_SIZE>) -> Self {
        core.set_in_sync(false);
        Self::Idle
    }

    pub fn tx<const SCRATCHPAD_SIZE: usize>(
        self,
        core: &mut SlaveCore<SCRATCHPAD_SIZE>,
    ) -> (Self, Option<u8>) {
        match self {
            Self::SendReadData { length, sent } => {
                let data = core.scratchpad()[sent as usize];
                core.update_crc_single(data);

                let state = if length >= sent {
                    SlaveState::SendCRC(core.crc().finalize())
                } else {
                    SlaveState::SendReadData {
                        length,
                        sent: sent + 1,
                    }
                };

                (state, Some(data))
            }
            Self::SendCRC(crc) => (Self::Idle, Some(crc)),
            x => (x, None),
        }
    }
}
