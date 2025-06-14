use crate::{
    command::Command,
    crc8::{CRC8Autosar, CRC},
    slave::{BusAction, BusState, CallbackAction, SlaveHandle},
    SINGLE_START_BYTE,
};

#[test]
pub fn one_length() {
    let mut slave = SlaveHandle::<1>::new_synced();
    let mut crc = CRC8Autosar::new().update_single_move(SINGLE_START_BYTE);

    slave.test_rx_single_start();
    slave.test_rx_no_response_no_callback(Command::BWR.u8());
    crc.update_single(Command::BWR.u8());

    // Offset High
    assert_eq!(
        slave.state,
        BusState::WriteOffsetH {
            crc: crc.clone(),
            respond: false
        },
        "BWR does not wait for high offset"
    );
    slave.test_rx_no_response_no_callback(0);
    crc.update_single(0);

    // Offset Low
    assert_eq!(
        slave.state,
        BusState::WriteOffsetL {
            crc: crc.clone(),
            offset: 0,
            respond: false
        },
        "BWR does not wait for low offset"
    );
    slave.test_rx_no_response_no_callback(0);
    crc.update_single(0);

    // Length
    assert_eq!(
        slave.state,
        BusState::WriteLength {
            crc: crc.clone(),
            respond: false,
            offset: 0
        },
        "BWR does not wait for length"
    );
    slave.test_rx_no_response_no_callback(0x01);
    crc.update_single(0x01);

    // Data: 0xAA
    assert_eq!(
        slave.state,
        BusState::WriteData {
            crc: crc.clone(),
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
        slave.state,
        BusState::WaitForCRC(crc.finalize(), BusAction::WriteAndIdle(0, 1)),
        "BWR does not wait for CRC"
    );
    fn callback(action: CallbackAction) -> bool {
        assert_eq!(action, CallbackAction::Write(0, &[0xaa]));
        true
    }
    slave.test_rx_no_response(crc.finalize(), &mut callback);

    // Idle
    assert_eq!(
        slave.state,
        BusState::Idle,
        "BWR does not go back to idle after write"
    );
}
