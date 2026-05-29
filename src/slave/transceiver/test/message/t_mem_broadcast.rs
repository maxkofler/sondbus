//! Tests that test memory commands in broadcast mode

use crate::{
    model::command::{
        Command, MemoryAddressingMode, MemoryCommand, Operation, SlaveAddressingMode,
    },
    slave::transceiver::{state::State, CallbackAction, Transceiver},
};

use super::super::new_transceiver;

/// Test writing 1 byte to memory in broadcast
#[test]
fn memory_8_w_1_broadcast() {
    const Q_OFFSET: u8 = 0x11;
    const Q_LEN: u8 = 1;

    let mut scratchpad = [0];
    let mut t = Transceiver::new_in_sync_sc0(&mut scratchpad, [0, 0, 0, 0, 0, 0], |a| match a {
        CallbackAction::WriteMemory { offset, data } => {
            assert_eq!(offset, Q_OFFSET as usize);
            assert_eq!(data.len(), Q_LEN as usize);
            assert_eq!(data[0], 0xAA);
            Ok(())
        }
        a => panic!("Called unexpected action: {a:?}"),
    });

    let cmd = Command::Memory(MemoryCommand {
        operation: Operation::Write,
        slave_addressing_mode: SlaveAddressingMode::Broadcast,
        memory_addressing_mode: MemoryAddressingMode::Bits8,
    });

    t.t_handle_no_response(cmd.into());

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response(Q_OFFSET);

    assert_eq!(t.state, State::MemoryLength);
    t.t_handle_no_response(Q_LEN);
    assert_eq!(t.state, State::MemoryHeaderCRC);

    t.t_handle_crc();

    assert_eq!(t.state, State::MemoryRXPayload);
    t.t_handle_no_response(0xAA);

    assert_eq!(t.state, State::CRC);
    t.t_handle_crc();

    assert_eq!(t.state, State::Idle);

    // After the operation we should see the new value in the scratchpad
    assert_eq!(scratchpad[0], 0xAA);
}

/// Test reading 1 byte from memory in broadcast
#[test]
fn memory_8_r_1_broadcast() {
    const Q_OFFSET: u8 = 0x11;
    const Q_LEN: u8 = 1;

    let mut scratchpad = [0];
    let mut t = Transceiver::new_in_sync_sc0(&mut scratchpad, [0, 0, 0, 0, 0, 0], |a| match a {
        CallbackAction::ReadMemory { offset, data } => {
            assert_eq!(offset, Q_OFFSET as usize);
            assert_eq!(data.len(), Q_LEN as usize);
            data[0] = 0xAA;
            Ok(())
        }
        a => panic!("Called unexpected action: {a:?}"),
    });

    let cmd = Command::Memory(MemoryCommand {
        operation: Operation::Read,
        slave_addressing_mode: SlaveAddressingMode::Broadcast,
        memory_addressing_mode: MemoryAddressingMode::Bits8,
    });

    t.t_handle_no_response(cmd.into());

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response(Q_OFFSET);

    assert_eq!(t.state, State::MemoryLength);
    t.t_handle_no_response(Q_LEN);

    assert_eq!(t.state, State::MemoryHeaderCRC);
    t.t_handle_crc();

    assert_eq!(t.state, State::MemoryTXPayload);
    assert_eq!(t.handle(None), Some(0xAA));

    assert_eq!(t.state, State::CRC);
    t.t_handle_crc();

    assert_eq!(t.state, State::Idle);
}

/// Test writing 0 bytes to memory in broadcast
#[test]
fn memory_8_w_0_broadcast() {
    new_transceiver!(t);

    let cmd = Command::Memory(MemoryCommand {
        operation: Operation::Write,
        slave_addressing_mode: SlaveAddressingMode::Broadcast,
        memory_addressing_mode: MemoryAddressingMode::Bits8,
    });

    t.t_handle_no_response(cmd.into());

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response(0x11);

    assert_eq!(t.state, State::MemoryLength);
    t.t_handle_no_response(0x0);

    assert_eq!(t.state, State::MemoryHeaderCRC);
    t.t_handle_crc();

    // There is no payload field

    assert_eq!(t.state, State::CRC);
    t.t_handle_crc();

    assert_eq!(t.state, State::Idle);
}

/// Test reading 0 bytes from memory in broadcast
#[test]
fn memory_8_r_0_broadcast() {
    new_transceiver!(t);

    let cmd = Command::Memory(MemoryCommand {
        operation: Operation::Read,
        slave_addressing_mode: SlaveAddressingMode::Broadcast,
        memory_addressing_mode: MemoryAddressingMode::Bits8,
    });

    t.t_handle_no_response(cmd.into());

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response(0x11);

    assert_eq!(t.state, State::MemoryLength);
    t.t_handle_no_response(0x0);

    assert_eq!(t.state, State::MemoryHeaderCRC);
    t.t_handle_crc();

    // There is no payload field

    assert_eq!(t.state, State::CRC);
    t.t_handle_crc();

    assert_eq!(t.state, State::Idle);
}
