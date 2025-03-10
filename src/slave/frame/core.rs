use crate::crc8::CRC8Autosar;

#[derive(Debug, Default)]
pub struct Core {
    pub crc: CRC8Autosar,
    pub in_sync: bool,
}
