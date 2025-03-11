use frame::SlaveFrame;
use replace_with::replace_with_or_abort_unchecked;

mod frame;

#[derive(Default)]
pub struct SlaveCore {
    in_sync: bool,
    my_mac: [u8; 6],
}

pub struct Slave {
    state: SlaveFrame,
    core: SlaveCore,
}

impl Slave {
    /// Create a new slave
    pub fn new() -> Self {
        Self {
            state: SlaveFrame::default(),
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

        /*unsafe {
            replace_with_or_abort_unchecked(&mut self.state, |state| {
                let response = state.tx(&mut self.core);
                ret = response.response;
                response.state
            })
        };*/

        ret
    }

    pub fn in_sync(&self) -> bool {
        self.state.core.in_sync
    }
}
