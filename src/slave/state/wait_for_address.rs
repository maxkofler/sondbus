use crate::{
    crc8::{CRC8Autosar, CRC},
    impl_state,
    slave::{Core, Handler, State, StateMachine},
    FrameType,
};

use super::WaitForLength;

pub struct WaitForAddress {
    pub ty: FrameType,
    pub crc: CRC8Autosar,
}

impl_state!(WaitForAddress, StateMachine::WaitForAddress);

impl WaitForAddress {
    pub fn new(ty: FrameType, crc: CRC8Autosar) -> Self {
        Self { ty, crc }
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
