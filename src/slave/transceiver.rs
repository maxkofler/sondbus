//! The transceiver implements the lowest layer of the sondbus communication protocol
//! and handles synchronization of the communication and memory access.

pub mod command;

#[cfg(test)]
mod test;

use crate::{
    crc8::{CRC8Autosar, CRC},
    SYNC_SEQUENCE,
};
use command::Command;

type StateFunction = fn(&mut TransceiverContext, rx: Option<u8>) -> Option<u8>;
const STATES: [StateFunction; 4] = [wait_for_start, wait_for_cmd, state_sync, wait_for_crc];

#[repr(usize)]
#[derive(Clone, PartialEq, Debug)]
enum State {
    WaitForStart = 0,
    WaitForCommand,
    Sync,
    WaitForCRC,
}

pub struct TransceiverContext {
    state: State,
    crc: CRC8Autosar,
    cur_cmd: Command,
    in_sync: bool,

    pos: u16,

    scratchpad: &'static [u8],
}

impl TransceiverContext {
    pub const fn new(scratchpad: &'static [u8]) -> Self {
        Self {
            state: State::WaitForStart,
            crc: CRC8Autosar::new_const(),
            cur_cmd: Command::new(0),
            in_sync: false,

            pos: 0,

            scratchpad,
        }
    }

    pub fn loose_sync(&mut self) {
        self.in_sync = false;
    }

    pub fn handle(&mut self, rx: Option<u8>) -> Option<u8> {
        let state = self.state.clone() as usize;
        let res = STATES[state](self, rx);

        if let Some(rx) = rx {
            self.crc.update_single(rx);
        }

        res
    }
}

fn wait_for_start(ctx: &mut TransceiverContext, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        if rx == 0x55 {
            ctx.state = State::WaitForCommand;
            ctx.crc = CRC8Autosar::new_const();
        }
    }

    None
}

fn wait_for_cmd(ctx: &mut TransceiverContext, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        ctx.cur_cmd = Command::new(rx);

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
        }
    }

    None
}

fn state_sync(ctx: &mut TransceiverContext, rx: Option<u8>) -> Option<u8> {
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

fn wait_for_crc(ctx: &mut TransceiverContext, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        ctx.state = State::WaitForStart;

        if rx != ctx.crc.finalize() {
            ctx.loose_sync();
        }
    }

    None
}
