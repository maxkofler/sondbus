use crate::{
    command::Command,
    crc8::CRC,
    slave::{BusAction, SlaveHandle, SlaveState},
};

#[test]
pub fn non_zero_length() {
    let address = 1;
    let offset = 2;
    let length = 1;

    let mut slave = SlaveHandle::<1>::new_synced();
    slave.core.set_logical_address(address);

    slave.test_rx_single_start();

    // Command
    slave.test_rx_no_response_no_callback(Command::LWR.u8());
    slave.assert_state(SlaveState::WriteLogicalAddress);

    // Address
    slave.test_rx_no_response_no_callback(address);
    slave.assert_state(SlaveState::WriteOffset {
        accept: true,
        respond: true,
    });

    // Offset
    slave.test_rx_no_response_no_callback(offset);
    slave.assert_state(SlaveState::WriteLength {
        accept: true,
        respond: true,
        offset,
    });

    // Length
    slave.test_rx_no_response_no_callback(length);
    slave.assert_state(SlaveState::WriteData {
        accept: true,
        respond: true,
        offset,
        length,
        written: 0,
    });

    // Data
    for byte in 0..length - 1 {
        slave.test_rx_no_response_no_callback(byte);
        slave.assert_state(SlaveState::WriteData {
            accept: true,
            respond: true,
            offset,
            length,
            written: byte + 1,
        })
    }

    // Last data
    slave.test_rx_no_response_no_callback(length);
    slave.assert_state(SlaveState::WaitForCRC(
        slave.core.crc().finalize(),
        BusAction::WriteAndRespondCRC(offset as u16, length),
    ));

    // CRC
    let mut callback_called = false;
    slave.test_rx_response(
        slave.core.crc().finalize(),
        &mut |_| {
            callback_called = true;
            true
        },
        slave
            .core
            .crc()
            .clone()
            .update_single_move(slave.core.crc().finalize())
            .finalize(),
    );
    assert!(callback_called, "LWR does not call the callback");
    slave.assert_state(SlaveState::Idle)
}

#[test]
pub fn zero_length() {
    let address = 1;
    let offset = 2;
    let length = 0;

    let mut slave = SlaveHandle::<1>::new_synced();
    slave.core.set_logical_address(address);

    slave.test_rx_single_start();

    // Command
    slave.test_rx_no_response_no_callback(Command::LWR.u8());
    slave.assert_state(SlaveState::WriteLogicalAddress);

    // Address
    slave.test_rx_no_response_no_callback(address);
    slave.assert_state(SlaveState::WriteOffset {
        accept: true,
        respond: true,
    });

    // Offset
    slave.test_rx_no_response_no_callback(offset);
    slave.assert_state(SlaveState::WriteLength {
        accept: true,
        respond: true,
        offset,
    });

    // Length
    slave.test_rx_no_response_no_callback(length);
    slave.assert_state(SlaveState::WaitForCRC(
        slave.core.crc().finalize(),
        BusAction::RespondCRC,
    ));

    // CRC
    slave.test_rx_no_callback(
        slave.core.crc().finalize(),
        slave
            .core
            .crc()
            .clone()
            .update_single_move(slave.core.crc().finalize())
            .finalize(),
    );
    slave.assert_state(SlaveState::Idle)
}
