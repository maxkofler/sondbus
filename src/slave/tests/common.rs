//! Common utility functions used for testing the slave

use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::{BusState, CallbackAction, SlaveCore, SlaveHandle},
    SINGLE_START_BYTE,
};

impl<const S: usize> SlaveHandle<S> {
    /// Tests receiving `data` with no response and callback expected
    /// # Arguments
    /// * `data` - The data to be received by the bus
    pub fn test_rx_no_response_no_callback(&mut self, data: u8) {
        let res = self.rx(data, rx_callback_panic);
        assert_eq!(res, None, "Slave responded when it shouldn't");
    }

    /// Tests receiving `data` with no response and callback expected
    /// # Arguments
    /// * `data` - The data to be received by the bus
    pub fn test_rx_multi_no_response_no_callback(&mut self, data: &[u8]) {
        for byte in data {
            self.test_rx_no_response_no_callback(*byte);
        }
    }

    /// Tests receiving `data` with no response expected and a custom callback
    /// # Arguments
    /// * `data` - The data to be received by the bus
    /// * `callback` - The callback to pass the `rx()` function
    pub fn test_rx_no_response<F: FnMut(CallbackAction) -> bool>(
        &mut self,
        data: u8,
        callback: &mut F,
    ) {
        let res = self.rx(data, callback);
        assert_eq!(res, None, "Slave responded when it shouldn't");
    }

    /// Tests receiving `data` with no response expected and a custom callback
    /// # Arguments
    /// * `data` - The data to be received by the bus
    /// * `callback` - The callback to pass the `rx()` function
    pub fn test_rx_multi_no_response<F: FnMut(CallbackAction) -> bool>(
        &mut self,
        data: &[u8],
        callback: &mut F,
    ) {
        for data in data {
            self.test_rx_no_response(*data, callback);
        }
    }

    /// Receives the SINGLE_START byte and asserts that no response is
    /// given and the state transitions correctly
    pub fn test_rx_single_start(&mut self) {
        self.test_rx_no_response_no_callback(SINGLE_START_BYTE);
        assert_eq!(
            self.state,
            BusState::WaitForCommand(CRC8Autosar::new().update_single_move(SINGLE_START_BYTE)),
            "Does not react to start byte"
        );
    }

    /// Creates a new `SlaveHandle` that is already in sync
    pub fn new_synced() -> Self {
        Self {
            state: BusState::Idle,
            core: SlaveCore {
                in_sync: true,
                scratchpad: [0; S],
            },
        }
    }

    /// Returns the current CRC of the bus
    pub fn cur_crc(&self) -> CRC8Autosar {
        match &self.state {
            BusState::Idle => CRC8Autosar::new(),
            BusState::WaitForCommand(crc) => crc.clone(),
            BusState::Sync(crc, _) => crc.clone(),
            BusState::WriteOffsetH { crc, respond: _ } => crc.clone(),
            BusState::WriteOffsetL {
                crc,
                respond: _,
                offset: _,
            } => crc.clone(),
            BusState::WriteLength {
                crc,
                respond: _,
                offset: _,
            } => crc.clone(),
            BusState::WriteData {
                crc,
                respond: _,
                offset: _,
                length: _,
                written: _,
            } => crc.clone(),
            BusState::WaitForCRC(crc, _) => (*crc).into(),
        }
    }
}

/// A callback that panics if called, informing that the callback
/// should never be called
pub fn rx_callback_panic(_: CallbackAction) -> bool {
    panic!("Callback was called when not allowed");
}
