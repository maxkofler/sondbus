use crate::{
    crc8::{CRC8Autosar, CRC},
    impl_handler, impl_rx_noop,
    slave::frame::{Core, HandleResponse, SlaveState, TXHandler, WaitForStart},
    FrameType, START_BYTE,
};

pub enum Response01Ping {
    Start {
        mac: [u8; 6],
    },
    Type {
        mac: [u8; 6],
    },
    PingerMAC {
        crc: CRC8Autosar,
        mac: [u8; 6],
        pos: u8,
    },
    MyMAC {
        crc: CRC8Autosar,
        pos: u8,
    },
    CRC {
        crc: u8,
    },
}

impl Response01Ping {
    pub fn new(mac: [u8; 6]) -> Self {
        Self::Start { mac }
    }
}

impl TXHandler for Response01Ping {
    fn tx(self, core: &mut crate::SlaveCore) -> crate::slave::frame::HandlerResponse {
        match self {
            Self::Start { mac } => (Self::Type { mac }.into(), Some(START_BYTE)).into(),
            Self::Type { mac } => (
                Self::PingerMAC {
                    crc: CRC8Autosar::new().update_move(&[START_BYTE, FrameType::Ping as u8]),
                    mac,
                    pos: 0,
                }
                .into(),
                Some(FrameType::Ping as u8),
            )
                .into(),
            Self::PingerMAC { crc, mac, pos } => {
                let data = mac[pos as usize];
                let crc = crc.update_single_move(data);
                let pos = pos + 1;

                if pos >= 6 {
                    (Self::MyMAC { crc, pos: 0 }.into(), Some(data)).into()
                } else {
                    (Self::PingerMAC { crc, mac, pos }.into(), Some(data)).into()
                }
            }
            Self::MyMAC { crc, pos } => {
                let data = core.my_mac[pos as usize];
                let crc = crc.update_single_move(data);
                let pos = pos + 1;

                if pos >= 6 {
                    (
                        Self::CRC {
                            crc: crc.finalize(),
                        }
                        .into(),
                        Some(data),
                    )
                        .into()
                } else {
                    (Self::MyMAC { crc, pos }.into(), Some(data)).into()
                }
            }
            Self::CRC { crc } => (WaitForStart {}.into(), Some(crc)).into(),
        }
    }
}

impl From<Response01Ping> for HandleResponse {
    fn from(value: Response01Ping) -> Self {
        HandleResponse::Ping(value)
    }
}

impl From<Response01Ping> for SlaveState {
    fn from(value: Response01Ping) -> Self {
        SlaveState::HandleResponse(Core(HandleResponse::Ping(value)))
    }
}

impl_rx_noop!(Response01Ping);
impl_handler!(Response01Ping);
