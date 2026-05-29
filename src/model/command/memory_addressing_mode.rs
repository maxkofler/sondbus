#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum MemoryAddressingMode {
    Bits8 = 0b00,
    Bits16 = 0b01,
    Bits32 = 0b10,
    Bits64 = 0b11,
}

impl From<u8> for MemoryAddressingMode {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0b00 => Self::Bits8,
            0b01 => Self::Bits16,
            0b10 => Self::Bits32,
            0b11 => Self::Bits64,
            _ => unreachable!(),
        }
    }
}

impl MemoryAddressingMode {
    pub fn octets(&self) -> u8 {
        match self {
            Self::Bits8 => 1,
            Self::Bits16 => 2,
            Self::Bits32 => 4,
            Self::Bits64 => 8,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::command::MemoryAddressingMode as DUT;

    #[test]
    fn spec_parse() {
        assert_eq!(DUT::from(0b00u8), DUT::Bits8);
        assert_eq!(DUT::from(0b01u8), DUT::Bits16);
        assert_eq!(DUT::from(0b10u8), DUT::Bits32);
        assert_eq!(DUT::from(0b11u8), DUT::Bits64);
    }

    #[test]
    fn spec_serialize() {
        assert_eq!(DUT::Bits8 as u8, 0b00);
        assert_eq!(DUT::Bits16 as u8, 0b01);
        assert_eq!(DUT::Bits32 as u8, 0b10);
        assert_eq!(DUT::Bits64 as u8, 0b11);
    }
}
