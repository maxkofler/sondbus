#![no_std]

/// The start byte for single-command frames
pub const SINGLE_START_BYTE: u8 = 0x55;

pub mod command;
pub mod crc8;
pub mod slave;
