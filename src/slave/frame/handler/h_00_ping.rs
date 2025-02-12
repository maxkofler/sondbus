use crate::{
    crc8::CRC8Autosar,
    slave::{frame::Handler, SlaveCore},
};

/// Handler for the `Ping` frame type (0x00)
pub struct Handler00Ping {
    _crc: CRC8Autosar,
}

impl Handler00Ping {
    /// Create a new instance of the ping handler
    /// # Arguments
    /// * `crc` - The CRC over the received bytes
    pub fn new(crc: CRC8Autosar) -> Self {
        Self { _crc: crc }
    }
}

impl Handler for Handler00Ping {
    fn rx(self, _data: u8, _core: &mut SlaveCore) -> crate::slave::frame::HandlerResponse {
        todo!()
    }

    fn tx(self, _core: &mut SlaveCore) -> crate::slave::frame::HandlerResponse {
        todo!()
    }
}
