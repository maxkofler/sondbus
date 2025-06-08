//! Sondbus command(s)

/// A sondbus command
#[repr(u8)]
#[derive(Debug)]
pub enum Command {
    /// No operation
    NOP = 0x00,

    /// Sync
    SYN = 0x10,

    /// Broadcast write
    BWR = 0x14,

    /// Physically addressed read
    PRD = 0x16,

    /// Physically addressed write
    PWR = 0x18,

    /// Logically addressed read
    LRD(u8),

    /// Logically addressed write
    LWR(u8),
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
            0x10 => Some(Self::SYN),
            0x14 => Some(Self::BWR),
            0x16 => Some(Self::PRD),
            0x18 => Some(Self::PWR),
            0x20..=0x2F => Some(Self::LRD(data & 0xF)),
            0x40..=0x4F => Some(Self::LWR(data & 0xF)),
            _ => None,
        }
    }
}
