//! The transceiver implements the lowest layer of the sondbus communication protocol
//! and handles synchronization of the communication and memory access.

#[cfg(test)]
mod test;

use crate::crc8::CRC8Autosar;

type StateFunction = fn(&mut TransceiverContext, rx: Option<u8>) -> Option<u8>;
const STATES: [StateFunction; 1] = [wait_for_start];

#[repr(usize)]
#[derive(Clone, PartialEq, Debug)]
enum State {
    WaitForStart = 0,
    WaitForCommand = 1,
}

pub struct TransceiverContext {
    state: State,
    crc: CRC8Autosar,
    scratchpad: &'static [u8],
}

impl TransceiverContext {
    pub const fn new(scratchpad: &'static [u8]) -> Self {
        Self {
            state: State::WaitForStart,
            crc: CRC8Autosar::new_const(),
            scratchpad,
        }
    }

    pub fn handle(&mut self, rx: Option<u8>) -> Option<u8> {
        let state = self.state.clone() as usize;
        STATES[state as usize](self, rx)
    }
}

fn wait_for_start(ctx: &mut TransceiverContext, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        if rx == 0x55 {
            ctx.state = State::WaitForCommand;
        }
    }

    None
}
