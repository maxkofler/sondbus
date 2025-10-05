use crate::slave::transceiver::Transceiver;

// Memory command: Wait for the slave address
pub fn state_mem_address(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    None
}

/// Memory command: Wait for the offset to read at or write to
pub fn state_mem_offset(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    None
}

/// Memory command: Wait for the size to read or write
pub fn state_mem_size(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    None
}

/// Memory command: Wait for the header CRC for read commands
pub fn state_mem_header_crc(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    None
}

pub fn state_mem_rx_payload(t: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    None
}

pub fn state_mem_tx_payload(tr: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    None
}
