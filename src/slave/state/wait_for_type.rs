use crate::slave::{Core, Handler, State, StateMachine};

pub struct WaitForType {}

impl State for WaitForType {
    fn to_state(self) -> crate::slave::StateMachine {
        StateMachine::WaitForType(Core { state: self })
    }
}
impl Handler for WaitForType {
    fn handle(self, byte: Option<u8>) -> (crate::slave::StateMachine, Option<u8>) {
        if let Some(byte) = byte {
            if byte == 0x55 {}
        }

        (self.to_state(), None)
    }
}

#[cfg(test)]
mod test {
    use crate::{slave::Handler, Slave};

    #[test]
    fn sond() {
        let slave = Slave::new();
        slave.handle(Some(0x55));
    }
}
