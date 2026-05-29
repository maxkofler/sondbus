use crate::slave::transceiver::{StateFunction, Transceiver};

mod s_crc;
mod s_idle;
mod s_mgt_sync;

mod s_mem_header_crc;
mod s_mem_length;
mod s_mem_offset;
mod s_mem_rx_payload;
mod s_mem_skip_payload;
mod s_mem_slave_address;
mod s_mem_tx_payload;

/// Enumerates the state functions that the control flow
/// jumps to for the individual states.
///
/// Make sure that the order is EXACTLY the same as in [State]
const STATES: [StateFunction; 10] = [
    s_idle::state_idle,
    s_crc::state_crc,
    s_mgt_sync::state_management_sync,
    s_mem_slave_address::state_memory_slave_address,
    s_mem_offset::state_memory_offset,
    s_mem_length::state_memory_length,
    s_mem_header_crc::state_memory_header_crc,
    s_mem_tx_payload::state_memory_tx_payload,
    s_mem_rx_payload::state_memory_rx_payload,
    s_mem_skip_payload::state_memory_skip_payload,
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

    MemorySlaveAddress,
    MemoryOffset,
    MemoryLength,
    MemoryHeaderCRC,
    MemoryTXPayload,
    MemoryRXPayload,
    MemorySkipPayload,
}

pub fn handle(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    STATES[t.state.clone() as usize](t, rx)
}
