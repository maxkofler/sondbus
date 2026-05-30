//! Tests that test memory commands in broadcast mode

use crate::{
    model::command::{
        CommandTemplate, MemoryAddressingMode, MemoryCommand, Operation, SlaveAddressingMode,
    },
    slave::transceiver::{state::State, Transceiver},
};

#[test]
fn memory_16_w_1_broadcast() {
    const Q_OFFSET: u16 = 0x1122;
    const Q_LEN: u8 = 1;

    let mut scratchpad = [0];
    let mut t = Transceiver::new_in_sync_sc0(&mut scratchpad, [0, 0, 0, 0, 0, 0], |_| Ok(()));

    let cmd = CommandTemplate::Memory(MemoryCommand {
        operation: Operation::Write,
        slave_addressing_mode: SlaveAddressingMode::Broadcast,
        memory_addressing_mode: MemoryAddressingMode::Bits16,
    });

    t.t_handle_no_response(cmd.into_u8(0));

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response((Q_OFFSET >> 8) as u8);

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response(Q_OFFSET as u8);

    assert_eq!(t.state, State::MemoryLength);
    t.t_handle_no_response(Q_LEN);
    assert_eq!(t.state, State::MemoryHeaderCRC);

    t.t_handle_crc();

    assert_eq!(t.state, State::MemoryRXPayload);
    t.t_handle_no_response(0xAA);

    assert_eq!(t.state, State::Crc);
    t.t_handle_crc();

    assert_eq!(t.state, State::Idle);

    // After the operation we should see the new value in the scratchpad
    assert_eq!(scratchpad[0], 0xAA);
}

#[test]
fn memory_32_w_1_broadcast() {
    const Q_OFFSET: u32 = 0x11223344;
    const Q_LEN: u8 = 1;

    let mut scratchpad = [0];
    let mut t = Transceiver::new_in_sync_sc0(&mut scratchpad, [0, 0, 0, 0, 0, 0], |_| Ok(()));

    let cmd = CommandTemplate::Memory(MemoryCommand {
        operation: Operation::Write,
        slave_addressing_mode: SlaveAddressingMode::Broadcast,
        memory_addressing_mode: MemoryAddressingMode::Bits32,
    });

    t.t_handle_no_response(cmd.into_u8(0));

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response((Q_OFFSET >> 24) as u8);

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response((Q_OFFSET >> 16) as u8);

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response((Q_OFFSET >> 8) as u8);

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response(Q_OFFSET as u8);

    assert_eq!(t.state, State::MemoryLength);
    t.t_handle_no_response(Q_LEN);
    assert_eq!(t.state, State::MemoryHeaderCRC);

    t.t_handle_crc();

    assert_eq!(t.state, State::MemoryRXPayload);
    t.t_handle_no_response(0xAA);

    assert_eq!(t.state, State::Crc);
    t.t_handle_crc();

    assert_eq!(t.state, State::Idle);

    // After the operation we should see the new value in the scratchpad
    assert_eq!(scratchpad[0], 0xAA);
}

#[test]
fn memory_64_w_1_broadcast() {
    const Q_OFFSET: u64 = 0x1122334455667788;
    const Q_LEN: u8 = 1;

    let mut scratchpad = [0];
    let mut t = Transceiver::new_in_sync_sc0(&mut scratchpad, [0, 0, 0, 0, 0, 0], |_| Ok(()));

    let cmd = CommandTemplate::Memory(MemoryCommand {
        operation: Operation::Write,
        slave_addressing_mode: SlaveAddressingMode::Broadcast,
        memory_addressing_mode: MemoryAddressingMode::Bits64,
    });

    t.t_handle_no_response(cmd.into_u8(0));

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response((Q_OFFSET >> 56) as u8);

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response((Q_OFFSET >> 48) as u8);

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response((Q_OFFSET >> 40) as u8);

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response((Q_OFFSET >> 32) as u8);

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response((Q_OFFSET >> 24) as u8);

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response((Q_OFFSET >> 16) as u8);

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response((Q_OFFSET >> 8) as u8);

    assert_eq!(t.state, State::MemoryOffset);
    t.t_handle_no_response(Q_OFFSET as u8);

    assert_eq!(t.state, State::MemoryLength);
    t.t_handle_no_response(Q_LEN);
    assert_eq!(t.state, State::MemoryHeaderCRC);

    t.t_handle_crc();

    assert_eq!(t.state, State::MemoryRXPayload);
    t.t_handle_no_response(0xAA);

    assert_eq!(t.state, State::Crc);
    t.t_handle_crc();

    assert_eq!(t.state, State::Idle);

    // After the operation we should see the new value in the scratchpad
    assert_eq!(scratchpad[0], 0xAA);
}
