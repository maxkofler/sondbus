use crate::{
    crc8::{CRC8Autosar, CRC},
    impl_state,
    slave::{Core, Handler, State, StateMachine},
    FrameType,
};

use super::WaitForCRC;

pub struct WaitForData {
    pub ty: FrameType,
    pub addr: u8,
    pub remaining: u8,
    pub crc: CRC8Autosar,
}

impl_state!(WaitForData, StateMachine::WaitForData);

impl WaitForData {
    pub fn new(ty: FrameType, addr: u8, remaining: u8, crc: CRC8Autosar) -> Self {
        Self {
            ty,
            addr,
            remaining,
            crc,
        }
    }
}

impl Handler for WaitForData {
    fn handle(mut self, byte: Option<u8>) -> (crate::slave::StateMachine, Option<u8>) {
        if let Some(byte) = byte {
            self.crc.update_single(byte);
            self.remaining -= 1;
            if self.remaining == 0 {
                return (
                    WaitForCRC::new(self.ty, self.addr, self.crc).to_state(),
                    None,
                );
            }
        }

        (self.to_state(), None)
    }
}
