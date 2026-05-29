use crate::slave::transceiver::{StateFunction, Transceiver};

mod s_00_idle;

/// Enumerates the state functions that the control flow
/// jumps to for the individual states.
///
/// Make sure that the order is EXACTLY the same as in [State]
const STATES: [StateFunction; 1] = [s_00_idle::state_idle];

/// Enumerates the possible states the [Transceiver] can be in
///
/// Make sure that the order is EXACTLY the same as in [STATES]
#[repr(usize)]
#[derive(Clone, PartialEq, Debug)]
pub enum State {
    /// The idle state of the transceiver that waits for a command
    /// to be received
    Idle,

    /// The transceiver waits for the closing CRC
    CRC,

    ManagementSync,
}

pub fn handle(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    STATES[t.state.clone() as usize](t, rx)
}
