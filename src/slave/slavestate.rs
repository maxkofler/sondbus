use crate::{
    crc8::{CRC8Autosar, CRC},
    FrameAction, FrameType, START_BYTE,
};

/// The underlying state the bus is in when being used
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum BusState {
    /// The slave waits for the start byte
    #[default]
    WaitForStart,
    /// The slave waits for the byte indicating the frame at hand
    WaitForType,
    /// Waits for the address field
    WaitForAddress {
        crc: CRC8Autosar,
        ty: Option<FrameType>,
    },
    /// Waits for the length field
    WaitForLength {
        crc: CRC8Autosar,
        ty: Option<FrameType>,
        addr: u8,
    },
    /// Handle incoming bytes as the data field
    HandleData {
        crc: CRC8Autosar,
        ty: Option<FrameType>,
        addr: u8,
        remaining: u8,
    },
    /// Wait for the finishing CRC
    WaitForCRC {
        ty: Option<FrameType>,
        crc: CRC8Autosar,
    },
}

impl BusState {
    pub const fn const_default() -> Self {
        Self::WaitForStart
    }

    /// Handle an incoming byte from the physical layer
    /// # Arguments
    /// * `data` - The data to handle
    /// # Returns
    /// A tuple of `self` and the data to respond, if any
    pub fn handle(self, data: u8) -> (Self, Option<u8>) {
        match self {
            //
            // Wait for the start byte, ignore all other bytes
            //
            Self::WaitForStart => match data {
                START_BYTE => (Self::WaitForType, None),
                _ => (Self::WaitForStart, None),
            },
            //
            // Wait for the type byte and try to derive it
            //
            Self::WaitForType => (
                Self::WaitForAddress {
                    crc: CRC8Autosar::new().update_move(&[START_BYTE, data]),
                    ty: FrameType::from_u8(data),
                },
                None,
            ),
            //
            // Wait for the address byte
            //
            Self::WaitForAddress { crc, ty } => (
                Self::WaitForLength {
                    crc: crc.update_single_move(data),
                    ty,
                    addr: data,
                },
                None,
            ),
            //
            // Wait for the data length byte
            //
            Self::WaitForLength { crc, ty, addr } => (
                match data {
                    0 => Self::WaitForCRC {
                        ty,
                        crc: crc.update_single_move(data),
                    },
                    data => Self::HandleData {
                        crc: crc.update_single_move(data),
                        ty,
                        addr,
                        remaining: data,
                    },
                },
                None,
            ),
            //
            // Process the incoming byte as part of the data section
            //
            Self::HandleData {
                crc,
                ty,
                addr,
                remaining,
            } => match remaining {
                0 => (
                    Self::WaitForCRC {
                        ty,
                        crc: crc.update_single_move(data),
                    },
                    None,
                ),
                _ => {
                    let ty = ty.map(|ty| ty.process(data, addr));
                    (
                        Self::HandleData {
                            crc: crc.update_single_move(data),
                            ty,
                            addr,
                            remaining: remaining - 1,
                        },
                        None,
                    )
                }
            },
            //
            // Wait for the CRC finalization
            //
            Self::WaitForCRC { crc, ty } => {
                if crc.finalize() == data {
                    match ty.map(|f| f.commit()) {
                        None => (Self::WaitForStart, Some(0xFF)),
                        Some(t) => match t {
                            FrameAction::None => (Self::WaitForStart, None),
                        },
                    }
                } else {
                    (Self::WaitForStart, Some(0xAA))
                }
            }
        }
    }

    /// Get the next byte to send over the bus, if available
    pub fn next(self) -> (Self, Option<u8>) {
        (self, None)
    }
}
