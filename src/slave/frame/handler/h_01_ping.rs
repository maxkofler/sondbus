use crate::crc8::CRC8Autosar;

/// Handler for the `Ping` frame type (0x01)
pub struct Handler01Ping {
    _crc: CRC8Autosar,
}

impl Handler01Ping {
    /// Create a new instance of the ping handler
    /// # Arguments
    /// * `crc` - The CRC over the received bytes
    pub fn new(crc: CRC8Autosar) -> Self {
        Self { _crc: crc }
    }
}
