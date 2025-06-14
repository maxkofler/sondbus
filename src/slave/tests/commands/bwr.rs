use crate::{
    command::Command,
    crc8::{CRC8Autosar, CRC},
    slave::{BusAction, CallbackAction, SlaveHandle, SlaveState},
    SINGLE_START_BYTE,
};

#[test]
pub fn one_length() {
    let mut slave = SlaveHandle::<1>::new_synced();
    let mut crc = CRC8Autosar::new().update_single_move(SINGLE_START_BYTE);

    slave.test_rx_single_start();
    slave.test_rx_no_response_no_callback(Command::BWR.u8());
    crc.update_single(Command::BWR.u8());

    // Offset
    assert_eq!(
        slave.state(),
        SlaveState::WriteOffset { respond: false },
        "BWR does not wait for high offset"
    );
    slave.test_rx_no_response_no_callback(0);
    crc.update_single(0);

    // Length
    assert_eq!(
        slave.state(),
        SlaveState::WriteLength {
            respond: false,
            offset: 0
        },
        "BWR does not wait for length"
    );
    slave.test_rx_no_response_no_callback(0x01);
    crc.update_single(0x01);

    // Data: 0xAA
    assert_eq!(
        slave.state(),
        SlaveState::WriteData {
            respond: false,
            offset: 0,
            length: 1,
            written: 0
        },
        "BWR does not wait for data"
    );
    slave.test_rx_no_response_no_callback(0xAA);
    crc.update_single(0xAA);

    // CRC
    assert_eq!(
        slave.state(),
        SlaveState::WaitForCRC(crc.finalize(), BusAction::WriteAndIdle(0, 1)),
        "BWR does not wait for CRC"
    );
    fn callback(action: CallbackAction) -> bool {
        assert_eq!(action, CallbackAction::Write(0, &[0xaa]));
        true
    }
    slave.test_rx_no_response(crc.finalize(), &mut callback);

    // Idle
    assert_eq!(
        slave.state(),
        SlaveState::Idle,
        "BWR does not go back to idle after write"
    );
}

#[test]
pub fn zero_length() {
    let mut slave = SlaveHandle::<1>::new_synced();
    let mut crc = CRC8Autosar::new().update_single_move(SINGLE_START_BYTE);

    slave.test_rx_single_start();
    slave.test_rx_no_response_no_callback(Command::BWR.u8());
    crc.update_single(Command::BWR.u8());

    // Offset
    assert_eq!(
        slave.state(),
        SlaveState::WriteOffset { respond: false },
        "BWR does not wait for high offset"
    );
    slave.test_rx_no_response_no_callback(0);
    crc.update_single(0);

    // Length
    assert_eq!(
        slave.state(),
        SlaveState::WriteLength {
            respond: false,
            offset: 0
        },
        "BWR does not wait for length"
    );
    slave.test_rx_no_response_no_callback(0x0);
    crc.update_single(0x0);

    // CRC
    assert_eq!(
        slave.state(),
        SlaveState::WaitForCRC(crc.finalize(), BusAction::None),
        "BWR does not wait for CRC"
    );
    slave.test_rx_no_response_no_callback(crc.finalize());

    // Idle
    assert_eq!(
        slave.state(),
        SlaveState::Idle,
        "BWR does not go back to idle after write"
    );
}
