//! Tests that test memory commands in physical mode

use crate::{
    model::command::{
        CommandTemplate, MemoryAddressingMode, MemoryCommand, Operation, SlaveAddressingMode,
    },
    slave::transceiver::state::State,
};

use super::super::new_transceiver;

/// Test writing 1 byte to memory in broadcast
#[test]
fn memory_8_w_1_physical() {
    const Q_OFFSET: u8 = 0x11;
    const Q_LEN: u8 = 1;
    const MAC_ADDR: [u8; 6] = [1, 2, 3, 4, 5, 6];

    new_transceiver!(t, 1, MAC_ADDR);

    let cmd = CommandTemplate::Memory(MemoryCommand {
        operation: Operation::Write,
        slave_addressing_mode: SlaveAddressingMode::Physical,
        memory_addressing_mode: MemoryAddressingMode::Bits8,
    });

    t.t_handle_no_response(cmd.into_u8(0));

    for b in MAC_ADDR {
        assert_eq!(t.state, State::MemorySlaveAddress);
        t.t_handle_no_response(b);
    }

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response(Q_OFFSET);

    assert_eq!(t.state, State::MemoryLength);
    t.t_handle_no_response(Q_LEN);
    assert_eq!(t.state, State::MemoryHeaderCRC);

    t.t_handle_crc();

    assert_eq!(t.state, State::MemoryRXPayload);
    t.t_handle_no_response(0xAA);

    assert_eq!(t.state, State::Crc);
    t.t_handle_crc();

    assert_eq!(t.state, State::Idle);
    assert_eq!(t.scratchpad[0], 0xAA);
}

/// Test writing 1 byte to memory in broadcast
#[test]
fn memory_8_w_1_physical_not_targeted() {
    const Q_OFFSET: u8 = 0x11;
    const Q_LEN: u8 = 1;
    const MAC_ADDR: [u8; 6] = [1, 2, 3, 4, 5, 6];

    new_transceiver!(t, 1, MAC_ADDR);

    let cmd = CommandTemplate::Memory(MemoryCommand {
        operation: Operation::Write,
        slave_addressing_mode: SlaveAddressingMode::Physical,
        memory_addressing_mode: MemoryAddressingMode::Bits8,
    });

    t.t_handle_no_response(cmd.into_u8(0));

    for _ in MAC_ADDR {
        assert_eq!(t.state, State::MemorySlaveAddress);
        t.t_handle_no_response(0);
    }

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response(Q_OFFSET);

    assert_eq!(t.state, State::MemoryLength);
    t.t_handle_no_response(Q_LEN);
    assert_eq!(t.state, State::MemoryHeaderCRC);

    t.t_handle_crc();

    assert_eq!(t.state, State::MemorySkipPayload);
    t.t_handle_no_response(0xAA);

    assert_eq!(t.state, State::Crc);
    t.t_handle_crc();

    assert_eq!(t.state, State::Idle);
    assert_eq!(t.scratchpad[0], 0x00);
}
