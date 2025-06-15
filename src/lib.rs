#![cfg_attr(not(feature = "std"), no_std)]

/// The start byte for single-command frames
pub const SINGLE_START_BYTE: u8 = 0x55;

pub const SERIAL_DEFAULT_BAUD: usize = 9600;

/// The `magic` field of the sync command
pub const SYNC_MAGIC: [u8; 15] = [
    0x1F, 0x2E, 0x3D, 0x4C, 0x5B, 0x6A, 0x79, 0x88, 0x97, 0xA6, 0xB5, 0xC4, 0xD3, 0xE2, 0xF1,
];

pub mod command;
pub mod crc8;
pub mod slave;

#[cfg(feature = "master")]
pub mod master;

#[cfg(test)]
mod tests {
    use crate::{SINGLE_START_BYTE, SYNC_MAGIC};

    /// Test that the start byte for single-command frames
    /// adheres to the specification
    #[test]
    fn spec_single_start_byte() {
        assert_eq!(
            SINGLE_START_BYTE, 0x55,
            "Single-Command Start byte is not conform"
        )
    }

    /// Test that the sync magic adheres to the specification
    #[test]
    fn spec_sync_magic() {
        assert_eq!(
            SYNC_MAGIC,
            [
                0x1F, 0x2E, 0x3D, 0x4C, 0x5B, 0x6A, 0x79, 0x88, 0x97, 0xA6, 0xB5, 0xC4, 0xD3, 0xE2,
                0xF1,
            ],
            "Sync magic does not adhere to spec",
        )
    }
}
