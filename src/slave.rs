use frame::{RXHandler, SlaveState, TXHandler};
use replace_with::replace_with_or_abort_unchecked;

mod frame;

#[derive(Default)]
pub struct SlaveCore {
    in_sync: bool,
}

pub struct Slave {
    state: SlaveState,
    core: SlaveCore,
}

impl Slave {
    /// Create a new slave
    pub fn new() -> Self {
        Self {
            state: SlaveState::default(),
            core: SlaveCore::default(),
        }
    }

    /// Handle an incoming byte from the physical
    /// # Arguments
    /// * `byte` - The received byte
    /// # Returns
    /// A byte to send to the bus, if some
    pub fn rx(&mut self, byte: u8) -> Option<u8> {
        let mut ret = None;

        unsafe {
            replace_with_or_abort_unchecked(&mut self.state, |state| {
                let response = state.rx(byte, &mut self.core);
                ret = response.response;
                response.state
            })
        };

        ret
    }

    /// Try to pull some data from the bus implementation
    /// to put on the bus
    /// # Returns
    /// A byte to send to the bus, if some
    pub fn tx(&mut self) -> Option<u8> {
        let mut ret = None;

        unsafe {
            replace_with_or_abort_unchecked(&mut self.state, |state| {
                let response = state.tx(&mut self.core);
                ret = response.response;
                response.state
            })
        };

        ret
    }
}
