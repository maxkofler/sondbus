use crate::{
    crc8::{CRC8Autosar, CRC},
    impl_state,
    slave::{Core, Handler, State, StateMachine},
    FrameType,
};

use super::{Ping, WaitForStart};

pub struct WaitForCRC {
    pub ty: FrameType,
    pub addr: u8,
    pub crc: u8,
}

impl_state!(WaitForCRC, StateMachine::WaitForCRC);

impl WaitForCRC {
    pub fn new(ty: FrameType, addr: u8, crc: CRC8Autosar) -> Self {
        let crc = crc.finalize();
        Self { ty, addr, crc }
    }
}

impl Handler for WaitForCRC {
    fn handle(self, byte: Option<u8>) -> (crate::slave::StateMachine, Option<u8>) {
        if let Some(byte) = byte {
            if self.crc == byte {
                match self.ty {
                    FrameType::Ping => return Ping::default().handle(None),
                    _ => return (WaitForStart {}.to_state(), None),
                }
            } else {
                return (WaitForStart {}.to_state(), None);
            }
        }

        (self.to_state(), None)
    }
}
