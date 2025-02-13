use crate::{
    crc8::CRC8Autosar,
    slave::frame::{HandlerResponse, WaitForCRC},
    SlaveCore,
};

/// Handler for the `Ping` frame type (0x01)
pub struct Handler01Ping {
    crc: CRC8Autosar,
}

impl Handler01Ping {
    /// Create a new instance of the ping handler
    /// # Arguments
    /// * `crc` - The CRC over the received bytes
    pub fn new(crc: CRC8Autosar) -> Self {
        Self { crc }
    }

    /// Sets up this handler by immediately transitioning
    /// to the CRC phase - this frame type does not have any data
    pub fn setup(self, _core: &mut SlaveCore) -> HandlerResponse {
        (WaitForCRC::new(self.crc).into(), None).into()
    }
}
