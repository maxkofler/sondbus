//! Structures and enumerations that describe the `Command` octet of a message

mod memory_addressing_mode;
mod slave_addressing_mode;

pub use memory_addressing_mode::MemoryAddressingMode;
pub use slave_addressing_mode::SlaveAddressingMode;

#[repr(u8)]
#[derive(Clone, PartialEq)]
pub enum Operation {
    Read = 0,
    Write = 1,
}
impl From<u8> for Operation {
    fn from(value: u8) -> Self {
        match value & 1 {
            0 => Self::Read,
            1 => Self::Write,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct MemoryCommand {
    pub operation: Operation,
    pub slave_addressing_mode: SlaveAddressingMode,
    pub memory_addressing_mode: MemoryAddressingMode,
}

#[derive(Clone)]
#[repr(u8)]
pub enum ManagementCommand {
    Nop = 0x00,
    Sync = 0x01,
}

pub enum CommandTemplate {
    Management(ManagementCommand),
    Memory(MemoryCommand),
}

#[derive(Clone)]
pub struct Command(pub u8);

impl Command {
    pub fn is_management(&self) -> bool {
        (self.0 >> 5) == 0
    }

    pub fn is_memory(&self) -> bool {
        !self.is_management()
    }

    pub fn get_manangement_command(&self) -> Result<Option<ManagementCommand>, ()> {
        if self.is_management() {
            Ok(Some(ManagementCommand::try_from(self.0)?))
        } else {
            Ok(None)
        }
    }

    pub fn is_memory_read(&self) -> bool {
        (self.0 & 1 << 4) == 0
    }

    pub fn mem_memory_addressing_bits(&self) -> u8 {
        self.0 & 0b11
    }

    pub fn mem_slave_addressing_bits(&self) -> u8 {
        (self.0 >> 2) & 0b11
    }

    pub fn mem_memory_addressing_octets(&self) -> u8 {
        1 << self.mem_memory_addressing_bits()
    }

    pub fn mem_slave_address_octets(&self) -> u8 {
        match self.mem_slave_addressing_bits() {
            0b00 => 0,
            0b01 => 6,
            0b10 => 2,
            0b11 => 0,
            _ => unreachable!(),
        }
    }
}

impl CommandTemplate {
    pub fn into_u8(self, sequence_counter: u8) -> u8 {
        match self {
            Self::Management(m) => m.into_u8(sequence_counter),
            Self::Memory(m) => m.into_u8(sequence_counter),
        }
    }
}

impl ManagementCommand {
    pub fn into_u8(self, sequence_counter: u8) -> u8 {
        ((sequence_counter & 0b11) << 6) | self as u8
    }
}

impl MemoryCommand {
    pub fn into_u8(self, sequence_counter: u8) -> u8 {
        ((sequence_counter & 0b11) << 6)
        | 1 << 5 // Memory command set
        | (self.operation as u8) << 4
        | (self.slave_addressing_mode as u8) << 2
        | self.memory_addressing_mode as u8
    }
}

impl TryFrom<u8> for ManagementCommand {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0b1_1111 {
            0 => Ok(Self::Nop),
            1 => Ok(Self::Sync),
            _ => Err(()),
        }
    }
}
