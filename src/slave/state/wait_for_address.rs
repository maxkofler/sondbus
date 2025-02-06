use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::{Core, Handler, State, StateMachine},
    FrameType,
};

use super::WaitForLength;

pub struct WaitForAddress {
    pub ty: FrameType,
    pub crc: CRC8Autosar,
}

impl WaitForAddress {
    pub fn new(ty: FrameType, crc: CRC8Autosar) -> Self {
        Self { ty, crc }
    }
}

impl State for WaitForAddress {
    fn to_state(self) -> crate::slave::StateMachine {
        StateMachine::WaitForAddress(Core { state: self })
    }
}

impl Handler for WaitForAddress {
    fn handle(self, byte: Option<u8>) -> (crate::slave::StateMachine, Option<u8>) {
        if let Some(byte) = byte {
            return (
                WaitForLength::new(self.ty, byte, self.crc.update_single_move(byte)).to_state(),
                None,
            );
        }

        (self.to_state(), None)
    }
}
