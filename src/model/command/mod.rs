//! Structures and enumerations that describe the `Command` octet of a message

mod memory_addressing_mode;
mod slave_addressing_mode;

pub use memory_addressing_mode::MemoryAddressingMode;
pub use slave_addressing_mode::SlaveAddressingMode;

#[repr(u8)]
#[derive(Clone)]
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
pub enum ManagementCommand {
    Nop = 0x00,
    Sync = 0x01,
}

#[derive(Clone)]
pub enum Command {
    Management(ManagementCommand),
    Memory(MemoryCommand),
}

impl TryFrom<u8> for Command {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let v = value & 0b11_1111;

        if (v & 1 << 5) == 0 {
            // Management command
            match v {
                0x00 => Ok(Self::Management(ManagementCommand::Nop)),
                0x01 => Ok(Self::Management(ManagementCommand::Sync)),
                x => Err(x),
            }
        } else {
            // Memory command
            Ok(Self::Memory(MemoryCommand {
                operation: (v >> 4).into(),
                slave_addressing_mode: (v >> 2).into(),
                memory_addressing_mode: v.into(),
            }))
        }
    }
}
