use replace_with::replace_with_or_abort;

use crate::slave::{BusState, CallbackAction, SlaveCore};

#[derive(Debug)]
pub struct SlaveHandle<const SCRATCHPAD_SIZE: usize> {
    state: BusState,
    pub core: SlaveCore<SCRATCHPAD_SIZE>,
}

impl<const SCRATCHPAD_SIZE: usize> SlaveHandle<SCRATCHPAD_SIZE> {
    /// A `const` variant of `default()` that allows const
    /// construction of a slave handle
    pub const fn default() -> Self {
        Self {
            state: BusState::Idle,
            core: SlaveCore::default(),
        }
    }

    /// Handle an incoming byte from the bus endpoint
    /// # Arguments
    /// * `data` - The byte of data to be handled
    /// * `callback` - A function that the bus can use to call
    ///                back to the host application for data reads and writes
    /// # Returns
    /// A possible byte to be sent back
    pub fn rx<F: FnMut(CallbackAction) -> bool>(&mut self, data: u8, callback: F) -> Option<u8> {
        let mut response = None;

        replace_with_or_abort(&mut self.state, |s| {
            let (s, r) = s.rx(data, &mut self.core, callback);
            response = r;
            s
        });

        response
    }

    /// Check if the bus has some data read to be sent
    /// # Returns
    /// A possible byte to be sent
    pub fn tx(&mut self) -> Option<u8> {
        let mut response = None;

        replace_with_or_abort(&mut self.state, |s| {
            let (s, r) = s.tx();
            response = r;
            s
        });

        response
    }

    /// Returns the sync state of the slave instance
    pub fn in_sync(&self) -> bool {
        self.core.in_sync()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        crc8::CRC8Autosar,
        slave::{
            tests::common::rx_callback_panic, BusState, CallbackAction, SlaveCore, SlaveHandle,
        },
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
                BusState::WaitForCommand,
                "Does not react to start byte"
            );
        }

        /// Creates a new `SlaveHandle` that is already in sync
        pub fn new_synced() -> Self {
            let mut core = SlaveCore::default();
            core.set_in_sync(true);
            Self {
                state: BusState::Idle,
                core,
            }
        }

        /// Returns the current CRC of the bus
        pub fn cur_crc(&self) -> CRC8Autosar {
            self.core.crc().clone()
        }

        pub fn state(&self) -> BusState {
            self.state.clone()
        }
    }
}
