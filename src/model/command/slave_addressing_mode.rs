#[repr(u8)]
#[derive(PartialEq, Debug, Clone)]
pub enum SlaveAddressingMode {
    Broadcast = 0b00,
    Physical = 0b01,
    Logical = 0b10,
    Virtual = 0b11,
}

impl From<u8> for SlaveAddressingMode {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0b00 => Self::Broadcast,
            0b01 => Self::Physical,
            0b10 => Self::Logical,
            0b11 => Self::Virtual,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::command::SlaveAddressingMode as DUT;

    #[test]
    fn spec_parse() {
        assert_eq!(DUT::from(0b00u8), DUT::Broadcast);
        assert_eq!(DUT::from(0b01u8), DUT::Physical);
        assert_eq!(DUT::from(0b10u8), DUT::Logical);
        assert_eq!(DUT::from(0b11u8), DUT::Virtual);
    }

    #[test]
    fn spec_serialize() {
        assert_eq!(DUT::Broadcast as u8, 0b00);
        assert_eq!(DUT::Physical as u8, 0b01);
        assert_eq!(DUT::Logical as u8, 0b10);
        assert_eq!(DUT::Virtual as u8, 0b11);
    }
}
