mod handle;
pub use handle::*;

mod core;
pub use core::*;

mod state;
pub use state::*;

/// The action to be performed when the bus
/// calls back to the host application
#[derive(Debug, PartialEq, Eq)]
pub enum CallbackAction<'a> {
    /// Write `1` at `0` to the memory
    Write(u16, &'a [u8]),

    /// Read from memory into `1` from `0`
    Read(u16, &'a mut [u8]),
}

/// The possible actions that follow a frame
#[derive(Clone, PartialEq, Debug, Default)]
pub enum BusAction {
    /// Do nothing
    #[default]
    None,

    /// Set the `in_sync` flag
    SetInSync(bool),

    /// Respond with the CRC
    WriteAndRespondCRC(u16, u8),

    /// Write the scratchpad data using the callback
    /// and go back to idle
    WriteAndIdle(u16, u8),

    RespondCRC,
    WaitForSecondCRC,
}

#[cfg(test)]
mod tests {
    pub mod common;

    pub mod commands {
        pub mod bwr;
        pub mod lwr;
    }

    use crate::{
        command::Command,
        crc8::{CRC8Autosar, CRC},
        slave::{tests::common::rx_callback_panic, BusAction, SlaveHandle, SlaveState},
        SINGLE_START_BYTE, SYNC_MAGIC,
    };

    /// Check that the bus correctly transitions from `Idle` to `WaitForCommand`
    /// when receiving a single-command start byte
    #[test]
    fn idle_to_cmd() {
        let mut slave = SlaveHandle::<0>::default();

        // This transition does NEVER yield a response
        assert_eq!(
            slave.rx(SINGLE_START_BYTE, rx_callback_panic),
            None,
            "Idle -> Command incorrectly responds"
        );

        // Check that the internal state moved to the correct next state
        assert_eq!(
            slave.state(),
            SlaveState::WaitForCommand,
            "Idle -> Command does not happen correctly"
        )
    }

    /// Test that the `in_sync` flag is kept as-is for a
    /// correct transition from `Idle` to `WaitForCommand`
    /// and that it is reset if an invalid byte is received
    #[test]
    fn idle_to_cmd_sync() {
        // Check that in_sync stays false for correct transitions
        let mut slave = SlaveHandle::<0>::default();
        assert!(
            !slave.core.in_sync(),
            "in_sync is not false for new instance"
        );
        slave.rx(SINGLE_START_BYTE, rx_callback_panic);
        assert!(!slave.core.in_sync(), "in_sync changed unexpectedly");

        // Check that in_sync stays true for correct transitions
        let mut slave = SlaveHandle::<0>::default();
        slave.core.set_in_sync(true);
        slave.rx(SINGLE_START_BYTE, rx_callback_panic);
        assert!(slave.core.in_sync(), "in_sync changed unexpectedly");

        // Check that in_sync is stays false for incorrect bytes
        let mut slave = SlaveHandle::<0>::default();
        slave.rx(SINGLE_START_BYTE + 0x34, rx_callback_panic);
        assert!(!slave.core.in_sync(), "in_sync is set for false starts");

        // Check that in_sync is set from true to false for incorrect bytes
        let mut slave = SlaveHandle::<0>::default();
        slave.core.set_in_sync(true);
        slave.rx(SINGLE_START_BYTE + 0x34, rx_callback_panic);
        assert!(
            !slave.core.in_sync(),
            "in_sync is not de-asserted for false starts"
        );
    }

    /// Test the `NOP` command
    #[test]
    fn cmd_nop() {
        let mut slave = SlaveHandle::<0>::default();
        slave.core.set_in_sync(true);

        let mut crc = CRC8Autosar::new();

        assert_eq!(slave.rx(SINGLE_START_BYTE, rx_callback_panic), None);
        crc.update_single(SINGLE_START_BYTE);
        assert_eq!(slave.state(), SlaveState::WaitForCommand);

        let cmd = Command::NOP.u8();
        assert_eq!(slave.rx(cmd, rx_callback_panic), None);
        crc.update_single(cmd);
        assert_eq!(
            slave.state(),
            SlaveState::WaitForCRC(crc.finalize(), BusAction::None)
        );
    }

    /// Test the `SYNC` command
    #[test]
    fn cmd_sync() {
        let mut slave = SlaveHandle::<0>::default();
        let mut crc = CRC8Autosar::new();

        crc.update(&[SINGLE_START_BYTE, Command::SYN.u8()]);

        assert_eq!(slave.rx(SINGLE_START_BYTE, rx_callback_panic), None);
        assert_eq!(slave.rx(Command::SYN.u8(), rx_callback_panic), None);

        for data in SYNC_MAGIC {
            assert_eq!(slave.rx(data, rx_callback_panic), None);
            crc.update_single(data);
        }

        assert_eq!(
            slave.state(),
            SlaveState::WaitForCRC(crc.finalize(), BusAction::SetInSync(true))
        );
        assert_eq!(slave.rx(crc.finalize(), rx_callback_panic), None);
        assert!(slave.core.in_sync())
    }
}
