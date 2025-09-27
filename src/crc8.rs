/// The initial value for the CRC calculation
pub const CRC8_AUTOSAR_INIT: u8 = 0xff;
/// The polynomial for computing the CRC checksum
pub const CRC8_AUTOSAR_POLY: u8 = 0x2f;
/// The value to XOR the final result with
pub const CRC8_AUTOSAR_XOR_OUT: u8 = 0xff;

pub trait CRC<T> {
    /// Creates a new CRC algorithm and computing instance
    fn new() -> Self;

    /// Resets the CRC computing instance as if it were recreated
    fn reset(&mut self);

    /// Update the CRC computing instance with the supplied byte
    /// # Arguments
    /// * `data` - The data to calculate into the CRC sum
    fn update_single(&mut self, data: u8);

    /// Updates the CRC and performing a move, returning `self`
    /// # Arguments
    /// * `data` - The data to update the CRC with
    fn update_single_move(mut self, data: u8) -> Self
    where
        Self: Sized,
    {
        self.update_single(data);
        self
    }

    /// Update the CRC computing instance with the supplied bytes
    /// # Arguments
    /// * `data` - The data to calculate into the CRC sum
    fn update(&mut self, data: &[u8]) {
        for d in data {
            self.update_single(*d);
        }
    }

    /// Updates the CRC and performing a move, returning `self`
    /// # Arguments
    /// * `data` - The data to update the CRC with
    fn update_move(mut self, data: &[u8]) -> Self
    where
        Self: Sized,
    {
        self.update(data);
        self
    }

    /// Non-destructively finalize the CRC, apply the last XOR and return
    /// the result
    ///
    /// This will not alter any values and can be called multiple times
    fn finalize(&self) -> T;
}

/// An implementation of the AUTOSAR CRC8 algorithm
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CRC8Autosar {
    pub crc: u8,
}

impl Default for CRC8Autosar {
    fn default() -> Self {
        Self::new()
    }
}

impl CRC8Autosar {
    pub const fn new_const() -> Self {
        Self {
            crc: CRC8_AUTOSAR_INIT,
        }
    }
}

impl CRC<u8> for CRC8Autosar {
    fn new() -> Self {
        Self::new_const()
    }

    fn reset(&mut self) {
        self.crc = CRC8_AUTOSAR_INIT;
    }

    fn update_single(&mut self, t: u8) {
        self.crc ^= t;
        for _ in 0..8 {
            if self.crc & 0x80 != 0 {
                self.crc = (self.crc << 1) ^ CRC8_AUTOSAR_POLY;
            } else {
                self.crc <<= 1;
            }
        }
    }

    fn finalize(&self) -> u8 {
        self.crc ^ CRC8_AUTOSAR_XOR_OUT
    }
}

impl From<u8> for CRC8Autosar {
    fn from(crc: u8) -> Self {
        Self { crc }
    }
}
