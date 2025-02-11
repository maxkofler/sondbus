use crate::{
    impl_state,
    slave::{Core, Handler, State, StateMachine},
    START_BYTE,
};

use super::WaitForStart;

impl_state!(Ping, StateMachine::FramePing);

#[derive(Default)]
pub struct Ping {
    state: PingState,
}

#[derive(Default)]
enum PingState {
    #[default]
    Start,

    Type,
    Address,
    Length,
    Data,
    Crc,
}

impl From<PingState> for Option<Ping> {
    fn from(state: PingState) -> Self {
        Some(Ping { state })
    }
}

impl Handler for Ping {
    fn handle(self, _: Option<u8>) -> (crate::slave::StateMachine, Option<u8>) {
        let (state, res): (Option<Ping>, Option<u8>) = match self.state {
            PingState::Start => (PingState::Type.into(), Some(START_BYTE)),
            PingState::Type => (PingState::Address.into(), Some(0x00)),
            PingState::Address => (PingState::Length.into(), Some(0x00)),
            PingState::Length => (PingState::Data.into(), Some(1)),
            PingState::Data => (PingState::Crc.into(), Some(0x55)),
            PingState::Crc => (None, Some(0xAA)),
        };

        match state {
            Some(state) => (state.to_state(), res),
            None => (WaitForStart {}.to_state(), res),
        }
    }
}
