use crate::{
    crc8::CRC,
    slave::transceiver::{Consequence, State, Transceiver},
};

pub fn state_mem_address(transceiver: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        let address_len = transceiver.cur_cmd.mem_slave_address_len();

        transceiver.mem_cmd_addr[transceiver.pos as usize] = rx;

        transceiver.pos += 1;

        if transceiver.pos as u8 >= address_len {
            // Next state
            println!("{}", transceiver.pos);
            transceiver.pos = 0;
            transceiver.state = State::MEMOffset;
        }
    }

    None
}

pub fn state_mem_offset(transceiver: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        let mut buf = transceiver.mem_cmd_offset.to_le_bytes();
        buf[transceiver.pos as usize] = rx;
        transceiver.mem_cmd_offset = u16::from_le_bytes(buf);

        transceiver.pos += 1;

        if transceiver.pos as u8 >= transceiver.cur_cmd.mem_offset_len() {
            transceiver.pos = 0;
            transceiver.state = State::MEMSize;
        }
    }

    None
}

pub fn state_mem_size(transceiver: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        let mut buf = transceiver.mem_cmd_size.to_le_bytes();
        buf[transceiver.pos as usize] = rx;
        transceiver.mem_cmd_size = u16::from_le_bytes(buf);

        transceiver.pos += 1;

        if transceiver.pos as u8 >= transceiver.cur_cmd.mem_size_len() {
            transceiver.pos = 0;

            // If we have a read command, we will expect a header CRC,
            // otherwise we will move on to the payload directly
            transceiver.state = match transceiver.cur_cmd.mem_is_read_cmd() {
                true => State::MEMHeaderCRC,
                false => State::MEMPayload,
            };
        }
    }
    None
}

pub fn state_mem_header_crc(transceiver: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        if rx == transceiver.crc.finalize() {
            transceiver.state = State::MEMPayload;
            transceiver.pos = 0;
        } else {
            transceiver.loose_sync();
            transceiver.state = State::WaitForStart;
        }
    }
    None
}

pub fn state_mem_payload(transceiver: &mut Transceiver, rx: Option<u8>) -> Option<u8> {
    if let Some(rx) = rx {
        transceiver.scratchpad[transceiver.pos as usize] = rx;

        transceiver.pos += 1;

        if transceiver.pos >= transceiver.mem_cmd_size {
            transceiver.pos = 0;

            if transceiver.is_targeted() {
                transceiver.consequence = Consequence::WriteScratchpad;
            }
            transceiver.state = State::WaitForCRC;
        }
    }
    None
}

impl Transceiver {
    fn is_targeted(&self) -> bool {
        if !self.cur_cmd.is_mem_cmd() {
            return false;
        }

        match self.cur_cmd.mem_slave_address_len() {
            0 => true,
            2 => &self.mem_cmd_addr[0..2] == self.logical_address,
            6 => self.mem_cmd_addr == self.physical_address,
            _ => false,
        }
    }
}
