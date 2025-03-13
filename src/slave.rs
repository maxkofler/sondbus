use frame::SlaveFrame;
use replace_with::replace_with_or_abort_unchecked;

mod callbacks;
pub use callbacks::*;

mod frame;

pub struct Slave {
    state: SlaveFrame,
}

impl Slave {
    /// Create a new slave
    pub fn new() -> Self {
        Self {
            state: SlaveFrame::default(),
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
                let response = state.rx(byte);
                ret = response.1;
                response.0
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
                let response = state.tx();
                ret = response.1;
                response.0
            })
        };

        ret
    }

    pub fn in_sync(&self) -> bool {
        self.state.core.in_sync
    }
}
