#![cfg_attr(not(test), no_std)]

pub mod crc8;
pub mod slave;

/// The byte sequence of the `SYNC` command
pub const SYNC_SEQUENCE: [u8; 15] = [
    0x1F, 0x2E, 0x3D, 0x4C, 0x5B, 0x6A, 0x79, 0x88, 0x97, 0xA6, 0xB5, 0xC4, 0xD3, 0xE2, 0xF1,
];

pub const START_BYTE: u8 = 0x55;
pub const CMD_NOP: u8 = 0x00;
pub const CMD_SYNC: u8 = 0x01;
pub const PROTOCOL_VERSION_1: u8 = 0x01;

macro_rules! test_log{
    ($($arg:tt)*) => {
        #[cfg(test)]
        println!($($arg)*)
    };
}

pub(crate) use test_log;
