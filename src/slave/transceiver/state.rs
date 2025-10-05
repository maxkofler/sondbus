use crate::slave::transceiver::{StateFunction, Transceiver};

mod state_mem_address;
mod state_mem_header_crc;
mod state_mem_offset;
mod state_mem_rx_payload;
mod state_mem_size;
mod state_mem_tx_payload;
mod state_send_crc;
mod state_sync;
mod state_wait_for_cmd;
mod state_wait_for_crc;
mod state_wait_for_start;

/// Enumerates the state functions that the control flow
/// jumps to for the individual states.
///
/// Make sure that the order is EXACTLY the same as in [State]
const STATES: [StateFunction; 11] = [
    state_wait_for_start::state_wait_for_start,
    state_wait_for_cmd::state_wait_for_cmd,
    state_sync::state_sync,
    state_mem_address::state_mem_address,
    state_mem_offset::state_mem_offset,
    state_mem_size::state_mem_size,
    state_mem_rx_payload::state_mem_rx_payload,
    state_mem_header_crc::state_mem_header_crc,
    state_mem_tx_payload::state_mem_tx_payload,
    state_send_crc::state_send_crc,
    state_wait_for_crc::state_wait_for_crc,
];

/// Enumerates the possible states the [Transceiver] can be in
///
/// Make sure that the order is EXACTLY the same as in [STATES]
#[repr(usize)]
#[derive(Clone, PartialEq, Debug)]
pub enum State {
    WaitForStart = 0,
    WaitForCommand,
    Sync,
    MEMAddress,
    MEMOffset,
    MEMSize,
    MEMRxPayload,
    MEMHeaderCRC,
    MEMTxPayload,
    SendCRC,
    WaitForCRC,
}

pub fn handle(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    STATES[t.state.clone() as usize](t, rx)
}
