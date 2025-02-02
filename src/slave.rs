mod state {
    mod wait_for_start;
    pub use wait_for_start::WaitForStart;

    mod wait_for_type;
    pub use wait_for_type::WaitForType;
}

use replace_with::replace_with_or_abort_unchecked;
use state::*;

pub struct Slave {
    state: StateMachine,
}

impl Slave {
    pub fn new() -> Self {
        Self {
            state: WaitForStart {}.to_state(),
        }
    }

    pub fn handle_mut(&mut self, byte: Option<u8>) -> Option<u8> {
        let mut ret = None;

        unsafe {
            replace_with_or_abort_unchecked(&mut self.state, |state| {
                let (s, r) = state.handle(byte);
                ret = r;
                s
            })
        };

        ret
    }
}

enum StateMachine {
    WaitForStart(Core<WaitForStart>),
    WaitForType(Core<WaitForType>),
}

trait State {
    fn to_state(self) -> StateMachine;
}

trait Handler {
    fn handle(self, byte: Option<u8>) -> (StateMachine, Option<u8>);
}

struct Core<S: State + Handler> {
    state: S,
}

impl Handler for StateMachine {
    fn handle(self, byte: Option<u8>) -> (StateMachine, Option<u8>) {
        match self {
            Self::WaitForStart(state) => state.state.handle(byte),
            Self::WaitForType(state) => state.state.handle(byte),
        }
    }
}

impl Handler for Slave {
    fn handle(self, byte: Option<u8>) -> (StateMachine, Option<u8>) {
        self.state.handle(byte)
    }
}

impl<S: State + Handler> State for Core<S> {
    fn to_state(self) -> StateMachine {
        self.state.to_state()
    }
}
