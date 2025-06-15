use crate::{
    command::Command,
    crc8::CRC,
    slave::{BusAction, CallbackAction, SlaveHandle, SlaveState},
};

#[test]
pub fn zero_length() {
    let address = 1;
    let offset = 2;
    let length = 0;

    let mut slave = SlaveHandle::<1>::new_synced();
    slave.core.set_logical_address(address);

    slave.test_rx_single_start();

    // Command
    slave.test_rx_no_response_no_callback(Command::LRD.u8());
    slave.assert_state(SlaveState::ReadLogicalAddress);

    // Address
    slave.test_rx_no_response_no_callback(address);
    slave.assert_state(SlaveState::ReadOffset { accept: true });

    // Offset
    slave.test_rx_no_response_no_callback(offset);
    slave.assert_state(SlaveState::ReadLength {
        accept: true,
        offset,
    });

    // Length
    slave.test_rx_no_response(length, &mut |_| true);
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
    slave.assert_state(SlaveState::Idle);
}

#[test]
pub fn one_length() {
    let address = 1;
    let offset = 2;
    let length = 1;

    let mut slave = SlaveHandle::<1>::new_synced();
    slave.core.set_logical_address(address);

    slave.test_rx_single_start();

    // Command
    slave.test_rx_no_response_no_callback(Command::LRD.u8());
    slave.assert_state(SlaveState::ReadLogicalAddress);

    // Address
    slave.test_rx_no_response_no_callback(address);
    slave.assert_state(SlaveState::ReadOffset { accept: true });

    // Offset
    slave.test_rx_no_response_no_callback(offset);
    slave.assert_state(SlaveState::ReadLength {
        accept: true,
        offset,
    });

    // Length
    slave.test_rx_no_response(length, &mut |_| true);
    slave.assert_state(SlaveState::WaitForCRC(
        slave.core.crc().finalize(),
        BusAction::ReadAndRespond(offset as u16, length),
    ));

    // CRC
    fn callback(action: CallbackAction) -> bool {
        if let CallbackAction::Read(_offset, data) = action {
            for (i, byte) in data.iter_mut().enumerate() {
                *byte = (i + 1) as u8;
            }
        } else {
            panic!();
        }

        true
    }
    slave.test_rx_response(slave.core.crc().finalize(), &mut callback, 1);
    slave.assert_state(SlaveState::SendCRC(slave.core.crc().finalize()));

    slave.test_tx_no_callback(slave.core.crc().finalize());
    slave.assert_state(SlaveState::Idle);
}

#[test]
pub fn non_zero_length() {
    let address = 1;
    let offset = 2;
    let length = 3;

    let mut slave = SlaveHandle::<3>::new_synced();
    slave.core.set_logical_address(address);

    slave.test_rx_single_start();

    // Command
    slave.test_rx_no_response_no_callback(Command::LRD.u8());
    slave.assert_state(SlaveState::ReadLogicalAddress);

    // Address
    slave.test_rx_no_response_no_callback(address);
    slave.assert_state(SlaveState::ReadOffset { accept: true });

    // Offset
    slave.test_rx_no_response_no_callback(offset);
    slave.assert_state(SlaveState::ReadLength {
        accept: true,
        offset,
    });

    // Length
    slave.test_rx_no_response(length, &mut |_| true);
    slave.assert_state(SlaveState::WaitForCRC(
        slave.core.crc().finalize(),
        BusAction::ReadAndRespond(offset as u16, length),
    ));

    // CRC
    fn callback(action: CallbackAction) -> bool {
        if let CallbackAction::Read(_offset, data) = action {
            for (i, byte) in data.iter_mut().enumerate() {
                *byte = (i + 1) as u8;
            }
        } else {
            panic!();
        }

        true
    }
    slave.test_rx_response(slave.core.crc().finalize(), &mut callback, 1);

    for i in 1..length - 1 {
        slave.assert_state(SlaveState::SendReadData { length: 3, sent: i });
        slave.test_tx_no_callback(i + 1);
    }

    slave.assert_state(SlaveState::SendCRC(slave.core.crc().finalize()));
    slave.test_tx_no_callback(slave.core.crc().finalize());
    slave.assert_state(SlaveState::Idle);
}

#[test]
pub fn non_zero_length_other_slave() {
    let address = 1;
    let offset = 2;
    let length = 3;

    let mut slave = SlaveHandle::<3>::new_synced();
    slave.core.set_logical_address(address - 1);

    slave.test_rx_single_start();

    // Command
    slave.test_rx_no_response_no_callback(Command::LRD.u8());
    slave.assert_state(SlaveState::ReadLogicalAddress);

    // Address
    slave.test_rx_no_response_no_callback(address);
    slave.assert_state(SlaveState::ReadOffset { accept: false });

    // Offset
    slave.test_rx_no_response_no_callback(offset);
    slave.assert_state(SlaveState::ReadLength {
        accept: false,
        offset,
    });

    // Length
    slave.test_rx_no_response_no_callback(length);
    slave.assert_state(SlaveState::WaitForCRC(
        slave.core.crc().finalize(),
        BusAction::WaitForOtherRead(length),
    ));

    // CRC
    slave.test_rx_no_response_no_callback(slave.core.crc().finalize());
    slave.test_rx_no_response_no_callback(1);

    for i in 1..length {
        slave.assert_state(SlaveState::ReceiveReadData { length: 3, sent: i });
        slave.test_rx_no_response_no_callback(i + 1);
    }

    slave.assert_state(SlaveState::WaitForCRC(
        slave.core.crc().finalize(),
        BusAction::None,
    ));
    slave.test_rx_no_response_no_callback(slave.core.crc().finalize());
    slave.assert_state(SlaveState::Idle);
}

#[test]
pub fn zero_length_other_slave() {
    let address = 1;
    let offset = 2;
    let length = 0;

    let mut slave = SlaveHandle::<3>::new_synced();
    slave.core.set_logical_address(address - 1);

    slave.test_rx_single_start();

    // Command
    slave.test_rx_no_response_no_callback(Command::LRD.u8());
    slave.assert_state(SlaveState::ReadLogicalAddress);

    // Address
    slave.test_rx_no_response_no_callback(address);
    slave.assert_state(SlaveState::ReadOffset { accept: false });

    // Offset
    slave.test_rx_no_response_no_callback(offset);
    slave.assert_state(SlaveState::ReadLength {
        accept: false,
        offset,
    });

    // Length
    slave.test_rx_no_response_no_callback(length);
    slave.assert_state(SlaveState::WaitForCRC(
        slave.core.crc().finalize(),
        BusAction::WaitForSecondCRC,
    ));

    // CRC
    slave.test_rx_no_response_no_callback(slave.core.crc().finalize());
    slave.assert_state(SlaveState::WaitForCRC(
        slave.core.crc().finalize(),
        BusAction::None,
    ));
    slave.test_rx_no_response_no_callback(slave.core.crc().finalize());
    slave.assert_state(SlaveState::Idle);
}
