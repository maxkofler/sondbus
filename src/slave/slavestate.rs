use crate::{
    crc8::{CRC8Autosar, CRC},
    frameaction::FrameAction,
    frametype::FrameType,
    Bus, START_BYTE,
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
    pub fn handle(self, data: u8, bus: &mut dyn Bus) -> (Self, Option<u8>) {
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
                    ty: FrameType::from_u8(data, bus),
                },
                None,
            ),
            //
            // Wait for the address byte
            //
            Self::WaitForAddress { crc, ty } => (
                Self::WaitForLength {
                    crc: crc.update_single_move(data),
                    ty: ty.map(|ty| ty.address(data)),
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
                        ty: ty.map(|ty| ty.length(data)),
                        crc: crc.update_single_move(data),
                    },
                    data => Self::HandleData {
                        crc: crc.update_single_move(data),
                        ty,
                        addr,
                        remaining: data - 1,
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
                    let ty = ty.map(|ty| ty.handle(data));
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
                    match ty.map(|f| f.commit(bus)) {
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

#[cfg(test)]
mod test {
    use crate::{
        crc8::{CRC8Autosar, CRC},
        Bus, START_BYTE,
    };

    use super::BusState;

    #[derive(Debug, Default)]
    pub struct DummyBus {}

    impl Bus for DummyBus {}

    #[test]
    fn waits_for_start() {
        let state = BusState::default();

        assert_eq!(
            state.clone().handle(0xFF, &mut DummyBus::default()).0,
            state,
            "Bus does not ignore non-start bytes"
        );
        assert_eq!(
            state.handle(START_BYTE, &mut DummyBus::default()).0,
            BusState::WaitForType,
            "Bus does not react to start byte"
        );
    }

    #[test]
    fn handles_empty_frame() {
        let mut state = BusState::default();

        let data = [START_BYTE, 0, 0, 0, 0];

        for byte in data {
            state = state.handle(byte, &mut DummyBus::default()).0;
        }

        assert_eq!(
            state,
            BusState::WaitForStart,
            "Bus does not handle an empty frame"
        )
    }

    #[test]
    fn crc_empty_frame() {
        let mut state = BusState::default();

        let data = [START_BYTE, 0, 0, 0];

        for byte in data {
            state = state.handle(byte, &mut DummyBus::default()).0;
        }

        if let BusState::WaitForCRC { ty: _, crc } = state {
            let real_crc = CRC8Autosar::new().update_move(&data);

            assert_eq!(real_crc.finalize(), crc.finalize());
        }
    }

    #[test]
    fn handles_filled_frame() {
        let mut state = BusState::default();

        for i in 0..=255u8 {
            let mut crc = CRC8Autosar::new();

            let data = [START_BYTE, 0, 0, i];
            for b in data {
                state = state.handle(b, &mut DummyBus::default()).0;
                crc.update_single(b);
            }

            assert_ne!(
                state,
                BusState::WaitForStart,
                "Bus does not exit idle state"
            );

            for b in 0..i {
                state = state.handle(b, &mut DummyBus::default()).0;
                crc.update_single(b);
            }

            state = state.handle(crc.finalize(), &mut DummyBus::default()).0;

            assert_eq!(
                state,
                BusState::WaitForStart,
                "Bus does not handle filled frame correctly"
            );
        }
    }
}
