//! The transceiver implements the lowest layer of the sondbus communication protocol
//! and handles synchronization of the communication and memory access.

mod command;

#[cfg(test)]
mod test;

use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::transceiver::command::memory::*,
    SYNC_SEQUENCE,
};
use command::Command;

type StateFunction = fn(&mut Transceiver, rx: Option<u8>) -> Option<u8>;

/// Enumerates the state functions that the control flow
/// jumps to for the individual states.
///
/// Make sure that the order is EXACTLY the same as in [State]
const STATES: [StateFunction; 10] = [
    state_wait_for_start,
    state_wait_for_cmd,
    state_sync,
    state_mem_address,
    state_mem_offset,
    state_mem_size,
    state_mem_header_crc,
    state_mem_payload,
    state_send_crc,
    state_wait_for_crc,
];

/// Enumerates the possible states the [Transceiver] can be in
///
/// Make sure that the order is EXACTLY the same as in [STATES]
#[repr(usize)]
#[derive(Clone, PartialEq, Debug)]
enum State {
    WaitForStart = 0,
    WaitForCommand,
    Sync,
    MEMAddress,
    MEMOffset,
    MEMSize,
    MEMHeaderCRC,
    MEMPayload,
    SendCRC,
    WaitForCRC,
}

/// Consequences of commands that are executed if a
/// command is finished with the right CRC
#[derive(PartialEq, Debug)]
enum Consequence {
    /// Nothing, return back to idle
    None,

    /// Write the contents of the scratchpad to the
    /// slave's memory area
    WriteScratchpad,
}

/// Represents a transceiver in the sondbus model.
///
/// The transceiver implements the lowest layer of the sondbus communication protocol
/// and handles synchronization of the communication and slave memory access.
pub struct Transceiver {
    state: State,
    crc: CRC8Autosar,
    cur_cmd: Command,
    in_sync: bool,

    pos: u16,
    mem_cmd_addr: [u8; 6],
    mem_cmd_offset: u16,
    mem_cmd_size: u16,

    physical_address: [u8; 6],
    logical_address: [u8; 2],

    scratchpad: &'static mut [u8],

    consequence: Consequence,
}

impl Transceiver {
    /// Creates a new transceiver
    /// # Arguments
    /// * `scratchpad` - The scratchpad memory to operate on
    pub const fn new(scratchpad: &'static mut [u8], physical_address: [u8; 6]) -> Self {
        Self {
            state: State::WaitForStart,
            crc: CRC8Autosar::new_const(),
            cur_cmd: Command::new(0),
            in_sync: false,

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

    /// Sets the internal `in_sync` flag false, effectively
    /// taking the transceiver offline until the next `Sync`
    /// command comes around from the master
    pub fn loose_sync(&mut self) {
        self.in_sync = false;
    }

    /// Process some event in the state machine of the transceiver.
    /// # Arguments
    /// * `rx` - An incoming byte from the physical layer
    /// # Returns
    /// A byte to be sent via the physical layer, if any
    pub fn handle(&mut self, rx: Option<u8>) -> Option<u8> {
        let state = self.state.clone() as usize;
        let res = STATES[state](self, rx);

        if let Some(rx) = rx {
            let old_crc = self.crc.finalize();
            self.crc.update_single(rx);
            println!(
                "RX {rx:x}, old CRC={old_crc:x}, new={:x}",
                self.crc.finalize()
            );
        }

        res
    }
}

fn state_wait_for_start(ctx: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        if rx == 0x55 {
            ctx.state = State::WaitForCommand;
            ctx.consequence = Consequence::None;
            ctx.crc = CRC8Autosar::new_const();
        }
    }

    None
}

fn state_wait_for_cmd(ctx: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        ctx.cur_cmd = Command::new(rx);
        ctx.pos = 0;

        if ctx.cur_cmd.is_mgt_cmd() {
            match ctx.cur_cmd.mgt_get_cmd() {
                // 0x00 is a NOP command, so we immediately wait for the CRC
                0x00 => {
                    ctx.state = State::WaitForCRC;
                }
                // 0x01 is the SYNC command, pass to the SYNC state
                0x01 => {
                    ctx.state = State::Sync;
                    ctx.pos = 0;
                }
                _ => {
                    ctx.loose_sync();
                    ctx.state = State::WaitForStart;
                }
            }
        } else {
            if ctx.cur_cmd.mem_slave_address_len() == 0 {
                ctx.state = State::MEMOffset;
            } else {
                ctx.state = State::MEMAddress;
            }
        }
    }

    None
}

fn state_sync(ctx: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        if ctx.pos <= 14 {
            if rx != SYNC_SEQUENCE[ctx.pos as usize] {
                ctx.loose_sync();
                ctx.state = State::WaitForStart;
            }
        } else if ctx.pos >= 15 {
            let version = rx;
            if version != 1 {
                ctx.loose_sync();
            } else {
                ctx.in_sync = true;
            }
            ctx.state = State::WaitForCRC;
        }

        ctx.pos += 1;
    }

    None
}

fn state_send_crc(t: &mut Transceiver, _rx: Option<u8>) -> Option<u8> {
    Some(t.crc.finalize())
}

fn state_wait_for_crc(ctx: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        ctx.state = State::WaitForStart;

        if rx != ctx.crc.finalize() {
            ctx.loose_sync();
        }
    }

    None
}
