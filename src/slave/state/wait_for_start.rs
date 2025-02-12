use crate::{
    crc8::{CRC8Autosar, CRC},
    impl_state,
    slave::{Core, Handler, State, StateMachine},
    START_BYTE,
};

use super::WaitForType;

pub struct WaitForStart {}

impl_state!(WaitForStart, StateMachine::WaitForStart);

impl Handler for WaitForStart {
    fn handle(self, byte: Option<u8>) -> (crate::slave::StateMachine, Option<u8>) {
        if let Some(byte) = byte {
            if byte == START_BYTE {
                return (
                    WaitForType::new(CRC8Autosar::new().update_single_move(byte)).to_state(),
                    None,
                );
            }
        }

        (self.to_state(), None)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        slave::{Handler, StateMachine},
        Slave, START_BYTE,
    };

    #[test]
    fn rx_start_byte() {
        let slave = Slave::new();
        let (state, response) = slave.handle(Some(START_BYTE));

        assert!(
            matches!(state, StateMachine::WaitForType(..)),
            "WaitForStart does not handle start byte"
        );
        assert!(response.is_none(), "WaitForStart returns data")
    }

    #[test]
    fn rx_other_byte() {
        let slave = Slave::new();
        let (state, response) = slave.handle(Some(0));

        assert!(
            matches!(state, StateMachine::WaitForStart(..)),
            "WaitForStart reacts to non-start-byte"
        );
        assert!(response.is_none(), "WaitForStart returns data")
    }

    #[test]
    fn handle_nothing() {
        let slave = Slave::new();
        let (state, response) = slave.handle(None);

        assert!(
            matches!(state, StateMachine::WaitForStart(..)),
            "WaitForStart reacts to no data"
        );
        assert!(response.is_none(), "WaitForStart returns data")
    }
}
