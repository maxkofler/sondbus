//! Sondbus command(s)

/// A sondbus command
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Command {
    /// No operation
    NOP = 0x00,

    /// Sync
    SYN = 0x01,

    /// Broadcast write
    BWR = 0x05,

    /// Physically addressed read
    PRD = 0x06,

    /// Physically addressed write
    PWR = 0x07,

    /// Logically addressed read
    LRD = 0x08,

    /// Logically addressed write
    LWR = 0x09,
}

impl Command {
    /// Tries to parse a command from `data`
    /// # Arguments
    /// * `data` - The byte to parse into a command
    /// # Returns
    /// The parsed command, if known
    pub fn from_u8(data: u8) -> Option<Self> {
        match data {
            0x00 => Some(Self::NOP),
            0x01 => Some(Self::SYN),
            0x05 => Some(Self::BWR),
            0x06 => Some(Self::PRD),
            0x07 => Some(Self::PWR),
            0x08 => Some(Self::LRD),
            0x09 => Some(Self::LWR),
            _ => None,
        }
    }

    /// Returns the `u8` representation for this command
    pub fn u8(self) -> u8 {
        self as u8
    }
}
