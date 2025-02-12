use crate::{
    crc8::{CRC8Autosar, CRC},
    impl_state,
    slave::{Core, Handler, State, StateMachine},
    FrameType,
};

use super::{WaitForAddress, WaitForStart};

pub struct WaitForType {
    pub crc: CRC8Autosar,
}

impl_state!(WaitForType, StateMachine::WaitForType);

impl WaitForType {
    pub fn new(crc: CRC8Autosar) -> Self {
        Self { crc }
    }
}

impl Handler for WaitForType {
    fn handle(self, byte: Option<u8>) -> (crate::slave::StateMachine, Option<u8>) {
        if let Some(byte) = byte {
            if let Some(ty) = FrameType::from_u8(byte) {
                return (
                    WaitForAddress::new(ty, self.crc.update_single_move(byte)).to_state(),
                    None,
                );
            } else {
                return (WaitForStart {}.to_state(), None);
            }
        }

        (self.to_state(), None)
    }
}
