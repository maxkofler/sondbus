use crate::slave::transceiver::{StateFunction, Transceiver};

mod s_crc;
mod s_idle;
mod s_mgt_sync;

/// Enumerates the state functions that the control flow
/// jumps to for the individual states.
///
/// Make sure that the order is EXACTLY the same as in [State]
const STATES: [StateFunction; 3] = [
    s_idle::state_idle,
    s_crc::state_crc,
    s_mgt_sync::state_management_sync,
];

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
