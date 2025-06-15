use crate::{
    command::Command,
    crc8::{CRC8Autosar, CRC},
    SINGLE_START_BYTE, SYNC_MAGIC,
};

pub mod blocking;
pub mod transport;

pub enum CommandDescriptor {
    SYN,

    BWR {
        offset: u8,
        data: Vec<u8>,
    },

    LWR {
        address: u8,
        offset: u8,
        data: Vec<u8>,
    },

    LRD {
        address: u8,
        offset: u8,
        data: Vec<u8>,
    },
}

/// The raw commands to be executed by an underlying
/// transport layer to reproduce a command
#[derive(Debug)]
pub struct CommandInstructions {
    /// The section that the master sends
    pub master_section: Vec<u8>,
    /// The sections that follow from the slave(s)
    pub slave_sections: Vec<SlaveSection>,
}

/// Describes a slave's turn on the bus
#[derive(Debug)]
pub struct SlaveSection {
    /// The amount of bytes it is expected to send
    pub bytes: usize,
    /// Whether or not to expect a CRC
    pub crc: bool,
}

impl CommandDescriptor {
    pub fn to_instructions(&self) -> CommandInstructions {
        let res = match self {
            Self::SYN => {
                let mut master_section = vec![SINGLE_START_BYTE, Command::SYN.u8()];
                master_section.extend(&SYNC_MAGIC);

                // The CRC
                master_section.push(CRC8Autosar::new().update_move(&master_section).finalize());

                CommandInstructions {
                    master_section,

                    slave_sections: Vec::new(),
                }
            }

            Self::BWR { offset, data } => {
                let mut master_section = vec![
                    SINGLE_START_BYTE,
                    Command::BWR.u8(),
                    *offset,
                    data.len() as u8,
                ];
                master_section.extend(data);

                // The CRC
                master_section.push(CRC8Autosar::new().update_move(&master_section).finalize());

                CommandInstructions {
                    master_section,

                    slave_sections: Vec::new(),
                }
            }

            Self::LWR {
                address,
                offset,
                data,
            } => {
                let mut master_section = vec![
                    SINGLE_START_BYTE,
                    Command::LWR.u8(),
                    *address,
                    *offset,
                    data.len() as u8,
                ];
                master_section.extend(data);

                // The CRC
                master_section.push(CRC8Autosar::new().update_move(&master_section).finalize());

                let slave_section = SlaveSection {
                    bytes: 0,
                    crc: true,
                };

                CommandInstructions {
                    master_section,
                    slave_sections: vec![slave_section],
                }
            }

            Self::LRD {
                address,
                offset,
                data,
            } => {
                let mut master_section = vec![
                    SINGLE_START_BYTE,
                    Command::LRD.u8(),
                    *address,
                    *offset,
                    data.len() as u8,
                ];

                // The CRC
                master_section.push(CRC8Autosar::new().update_move(&master_section).finalize());

                let slave_section = SlaveSection {
                    bytes: data.len(),
                    crc: true,
                };

                CommandInstructions {
                    master_section,
                    slave_sections: vec![slave_section],
                }
            }
        };

        res
    }

    pub fn populate_from_slave_sections(&mut self, sections: Vec<Vec<u8>>) {
        match self {
            Self::LRD {
                address,
                offset,
                data,
            } => {
                data.copy_from_slice(&sections[0]);
            }
            _ => {}
        }
    }
}
