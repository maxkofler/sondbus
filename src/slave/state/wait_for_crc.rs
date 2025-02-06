use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::{Core, Handler, State, StateMachine},
    FrameType,
};

use super::WaitForStart;

pub struct WaitForCRC {
    pub ty: FrameType,
    pub addr: u8,
    pub crc: u8,
}

impl WaitForCRC {
    pub fn new(ty: FrameType, addr: u8, crc: CRC8Autosar) -> Self {
        let crc = crc.finalize();
        Self { ty, addr, crc }
    }
}

impl State for WaitForCRC {
    fn to_state(self) -> crate::slave::StateMachine {
        StateMachine::WaitForCRC(Core { state: self })
    }
}

impl Handler for WaitForCRC {
    fn handle(self, byte: Option<u8>) -> (crate::slave::StateMachine, Option<u8>) {
        if let Some(byte) = byte {
            if self.crc == byte {
                return (WaitForStart {}.to_state(), Some(0x55));
            } else {
                return (WaitForStart {}.to_state(), Some(0xBB));
            }
        }

        (self.to_state(), None)
    }
}
