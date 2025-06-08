#![no_std]

/// The start byte for single-command frames
pub const SINGLE_START_BYTE: u8 = 0x55;

pub mod command;
pub mod crc8;
pub mod slave;

#[cfg(test)]
mod tests {
    use crate::SINGLE_START_BYTE;

    /// Test that the start byte for single-command frames
    /// adheres to the specification
    #[test]
    fn spec_single_start_byte() {
        assert_eq!(
            SINGLE_START_BYTE, 0x55,
            "Single-Command Start byte is not conform"
        )
    }
}
