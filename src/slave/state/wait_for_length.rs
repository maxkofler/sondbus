use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::{Core, Handler, State, StateMachine},
    FrameType,
};

use super::{WaitForData, WaitForStart};

pub struct WaitForLength {
    pub ty: FrameType,
    pub addr: u8,
    pub crc: CRC8Autosar,
}

impl WaitForLength {
    pub fn new(ty: FrameType, addr: u8, crc: CRC8Autosar) -> Self {
        Self { ty, addr, crc }
    }
}

impl State for WaitForLength {
    fn to_state(self) -> crate::slave::StateMachine {
        StateMachine::WaitForLength(Core { state: self })
    }
}

impl Handler for WaitForLength {
    fn handle(self, byte: Option<u8>) -> (crate::slave::StateMachine, Option<u8>) {
        if let Some(byte) = byte {
            if byte == 0 {
                return (WaitForStart {}.to_state(), None);
            } else {
                return (
                    WaitForData::new(self.ty, self.addr, byte, self.crc.update_single_move(byte))
                        .to_state(),
                    None,
                );
            }
        }

        (self.to_state(), None)
    }
}
