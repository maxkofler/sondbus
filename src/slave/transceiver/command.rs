pub struct Command {
    value: u8,
}

impl Command {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn is_mem_cmd(&self) -> bool {
        (self.value & 1 << 5) != 0
    }

    pub fn is_mgt_cmd(&self) -> bool {
        (self.value & 1 << 5) == 0
    }

    pub fn mgt_get_cmd(&self) -> u8 {
        self.value & 0b11111
    }

    pub fn mem_is_read_cmd(&self) -> bool {
        self.value & 1 == 0
    }

    pub fn mem_is_write_cmd(&self) -> bool {
        self.value & 1 != 0
    }

    pub fn mem_slave_address_len(&self) -> u8 {
        // Match on the addressing mode selector bits (1+2)
        match (self.value >> 1) & 0b11 {
            1 => 6, // Addressed by physical address
            2 => 2, // Addressed by logical address
            _ => 0, // We're not addressed at all (broadcast or logical memory)
        }
    }

    pub fn mem_offset_len(&self) -> u8 {
        match (self.value >> 3) & 1 {
            1 => 2,
            _ => 1,
        }
    }

    pub fn mem_size_len(&self) -> u8 {
        match (self.value >> 4) & 1 {
            1 => 2,
            _ => 1,
        }
    }
}
