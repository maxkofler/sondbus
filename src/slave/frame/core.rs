use crate::crc8::CRC8Autosar;

#[derive(Debug, Default)]
pub struct Core {
    pub crc: CRC8Autosar,
    pub my_mac: [u8; 6],
    pub in_sync: bool,
}
