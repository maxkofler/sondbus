use handler::Handler01Ping;

use crate::{
    crc8::{CRC8Autosar, CRC},
    FrameType, START_BYTE,
};

use super::SlaveCore;

mod handler {
    mod h_01_ping;
    pub use h_01_ping::Handler01Ping;
}

/// The core of the slave state machine to house the slave state
pub struct Core<S: Handler + Into<SlaveState>>(S);

/// The state for the slave
pub enum SlaveState {
    WaitForStart(Core<WaitForStart>),
    WaitForType(Core<WaitForType>),
    HandleData(Core<HandleData>),
}

/// A response from a handler function
pub struct HandlerResponse {
    /// The new state to transition to
    pub state: SlaveState,
    /// The response to the bus from the handler, if some
    pub response: Option<u8>,
}

/// A trait for all handlers of a state
pub trait Handler {
    fn rx(self, data: u8, core: &mut SlaveCore) -> HandlerResponse;
    fn tx(self, core: &mut SlaveCore) -> HandlerResponse;
}

/// State: Wait for start
///
/// Waits for the start byte to be received before transitioning
/// to [WaitForType].
pub struct WaitForStart {}

impl Handler for WaitForStart {
    fn rx(self, data: u8, _core: &mut SlaveCore) -> HandlerResponse {
        if data == START_BYTE {
            let crc = CRC8Autosar::new().update_single_move(START_BYTE);
            (WaitForType { crc }.into(), None).into()
        } else {
            (self.into(), None).into()
        }
    }

    fn tx(self, _core: &mut SlaveCore) -> HandlerResponse {
        (self.into(), None).into()
    }
}

/// State: Wait for the frame type
///
/// Waits for the frame type to be received and forwards
/// the correct handler if implemented.
pub struct WaitForType {
    crc: CRC8Autosar,
}

impl Handler for WaitForType {
    fn rx(self, data: u8, _core: &mut SlaveCore) -> HandlerResponse {
        match FrameType::from_u8(data) {
            None => (WaitForStart {}.into(), None).into(),
            Some(ty) => (ty.to_handler(self.crc).into(), None).into(),
        }
    }

    fn tx(self, _core: &mut SlaveCore) -> HandlerResponse {
        (self.into(), None).into()
    }
}

/// State: Handle incoming data
///
/// Forwards incoming data to the corresponding handler to
/// process and yield a new state.
pub enum HandleData {
    /// Handle the `Ping` frame type (0x01)
    Ping(Handler01Ping),
}

impl Handler for HandleData {
    fn rx(self, data: u8, core: &mut SlaveCore) -> HandlerResponse {
        match self {
            Self::Ping(handler) => handler.rx(data, core),
        }
    }

    fn tx(self, core: &mut SlaveCore) -> HandlerResponse {
        match self {
            Self::Ping(handler) => handler.tx(core),
        }
    }
}

//
//
// =================        Boilerplate code        =================
//
//

impl FrameType {
    /// Create a handler from this frame type to be used in the
    /// slave [HandleData] state.
    /// # Arguments
    /// * `crc` - The CRC checksum up until this point
    fn to_handler(self, crc: CRC8Autosar) -> HandleData {
        match self {
            FrameType::Ping => HandleData::Ping(Handler01Ping::new(crc)),
        }
    }
}

impl From<WaitForStart> for SlaveState {
    fn from(value: WaitForStart) -> Self {
        Self::WaitForStart(Core(value))
    }
}

impl From<WaitForType> for SlaveState {
    fn from(value: WaitForType) -> Self {
        Self::WaitForType(Core(value))
    }
}

impl From<HandleData> for SlaveState {
    fn from(value: HandleData) -> Self {
        Self::HandleData(Core(value))
    }
}

impl Handler for SlaveState {
    fn rx(self, data: u8, core: &mut SlaveCore) -> HandlerResponse {
        match self {
            Self::WaitForStart(state) => state.0.rx(data, core),
            Self::WaitForType(state) => state.0.rx(data, core),
            Self::HandleData(state) => state.0.rx(data, core),
        }
    }

    fn tx(self, core: &mut SlaveCore) -> HandlerResponse {
        match self {
            Self::WaitForStart(state) => state.0.tx(core),
            Self::WaitForType(state) => state.0.tx(core),
            Self::HandleData(state) => state.0.tx(core),
        }
    }
}

impl From<(SlaveState, Option<u8>)> for HandlerResponse {
    fn from(value: (SlaveState, Option<u8>)) -> Self {
        Self {
            state: value.0,
            response: value.1,
        }
    }
}

impl From<SlaveState> for HandlerResponse {
    fn from(state: SlaveState) -> Self {
        Self {
            state,
            response: None,
        }
    }
}
