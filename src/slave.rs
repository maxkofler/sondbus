mod state {
    mod wait_for_start;
    pub use wait_for_start::WaitForStart;

    mod wait_for_type;
    pub use wait_for_type::WaitForType;

    mod wait_for_address;
    pub use wait_for_address::WaitForAddress;

    mod wait_for_length;
    pub use wait_for_length::WaitForLength;

    mod wait_for_data;
    pub use wait_for_data::WaitForData;

    mod wait_for_crc;
    pub use wait_for_crc::WaitForCRC;

    mod frame_ping;
    pub use frame_ping::Ping;
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
    WaitForAddress(Core<WaitForAddress>),
    WaitForLength(Core<WaitForLength>),
    WaitForData(Core<WaitForData>),
    WaitForCRC(Core<WaitForCRC>),
    FramePing(Core<Ping>),
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
            Self::WaitForAddress(state) => state.state.handle(byte),
            Self::WaitForLength(state) => state.state.handle(byte),
            Self::WaitForData(state) => state.state.handle(byte),
            Self::WaitForCRC(state) => state.state.handle(byte),
            Self::FramePing(state) => state.state.handle(byte),
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

#[macro_export]
macro_rules! impl_state {
    ($x: ty, $y: expr) => {
        impl State for $x {
            fn to_state(self) -> crate::slave::StateMachine {
                $y(Core { state: self })
            }
        }
    };
}
