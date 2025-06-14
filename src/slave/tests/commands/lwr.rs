use crate::{
    command::Command,
    crc8::{CRC8Autosar, CRC},
    slave::{BusAction, CallbackAction, SlaveHandle, SlaveState},
    SINGLE_START_BYTE,
};

#[test]
pub fn one_length() {
    let mut slave = SlaveHandle::<1>::new_synced();
    slave.core.set_logical_address(1);
    let mut crc = CRC8Autosar::new().update_single_move(SINGLE_START_BYTE);

    slave.test_rx_single_start();
    slave.test_rx_no_response_no_callback(Command::LWR.u8());
    crc.update_single(Command::LWR.u8());

    // Address
    assert_eq!(
        slave.state(),
        SlaveState::WriteLogicalAddress,
        "LWR does not wait for address"
    );
    slave.test_rx_no_response_no_callback(1);
    crc.update_single(1);

    // Offset
    assert_eq!(
        slave.state(),
        SlaveState::WriteOffset {
            accept: true,
            respond: true
        },
        "LWR does not wait for high offset"
    );
    slave.test_rx_no_response_no_callback(0);
    crc.update_single(0);

    // Length
    assert_eq!(
        slave.state(),
        SlaveState::WriteLength {
            accept: true,
            respond: true,
            offset: 0
        },
        "LWR does not wait for length"
    );
    slave.test_rx_no_response_no_callback(0x01);
    crc.update_single(0x01);

    // Data: 0xAA
    assert_eq!(
        slave.state(),
        SlaveState::WriteData {
            accept: true,
            respond: true,
            offset: 0,
            length: 1,
            written: 0
        },
        "LWR does not wait for data"
    );
    slave.test_rx_no_response_no_callback(0xAA);
    crc.update_single(0xAA);

    // CRC Master
    assert_eq!(
        slave.state(),
        SlaveState::WaitForCRC(crc.finalize(), BusAction::WriteAndRespondCRC(0, 1)),
        "LWR does not wait for CRC"
    );
    fn callback(action: CallbackAction) -> bool {
        assert_eq!(action, CallbackAction::Write(0, &[0xaa]));
        true
    }
    slave.test_rx_response(
        crc.finalize(),
        &mut callback,
        crc.clone().update_single_move(crc.finalize()).finalize(),
    );

    // Idle
    assert_eq!(
        slave.state(),
        SlaveState::Idle,
        "LWR does not go back to idle after write"
    );
}

#[test]
pub fn zero_length() {
    let mut slave = SlaveHandle::<1>::new_synced();
    slave.core.set_logical_address(1);
    let mut crc = CRC8Autosar::new().update_single_move(SINGLE_START_BYTE);

    slave.test_rx_single_start();
    slave.test_rx_no_response_no_callback(Command::LWR.u8());
    crc.update_single(Command::LWR.u8());

    // Address
    assert_eq!(
        slave.state(),
        SlaveState::WriteLogicalAddress,
        "LWR does not wait for address"
    );
    slave.test_rx_no_response_no_callback(1);
    crc.update_single(1);

    // Offset
    assert_eq!(
        slave.state(),
        SlaveState::WriteOffset {
            accept: true,
            respond: true
        },
        "LWR does not wait for high offset"
    );
    slave.test_rx_no_response_no_callback(0);
    crc.update_single(0);

    // Length
    assert_eq!(
        slave.state(),
        SlaveState::WriteLength {
            accept: true,
            respond: true,
            offset: 0
        },
        "LWR does not wait for length"
    );
    slave.test_rx_no_response_no_callback(0x0);
    crc.update_single(0x0);

    // CRC Master
    assert_eq!(
        slave.state(),
        SlaveState::WaitForCRC(crc.finalize(), BusAction::RespondCRC),
        "LWR does not wait for CRC"
    );
    fn callback(action: CallbackAction) -> bool {
        assert_eq!(action, CallbackAction::Write(0, &[0xaa]));
        true
    }
    slave.test_rx_response(
        crc.finalize(),
        &mut callback,
        crc.clone().update_single_move(crc.finalize()).finalize(),
    );

    // Idle
    assert_eq!(
        slave.state(),
        SlaveState::Idle,
        "LWR does not go back to idle after write"
    );
}
