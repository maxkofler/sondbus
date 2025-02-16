use crate::{
    crc8::{CRC8Autosar, CRC},
    impl_handler, impl_rx_noop,
    slave::frame::{Core, HandleResponse, SlaveState, TXHandler, WaitForStart},
    FrameType, START_BYTE,
};

use super::array::{OwnedArraySender, OwnedArraySenderResult};

pub enum Response01Ping {
    Start {
        mac: [u8; 6],
    },
    Type {
        mac: [u8; 6],
    },
    PingerMAC {
        crc: CRC8Autosar,
        sender: OwnedArraySender<6>,
    },
    MyMAC {
        crc: CRC8Autosar,
        sender: OwnedArraySender<6>,
    },
    Crc {
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
                    sender: OwnedArraySender::new(mac),
                }
                .into(),
                Some(FrameType::Ping as u8),
            )
                .into(),

            Self::PingerMAC { crc, sender } => {
                let (state, res) = sender.handle();
                let crc = crc.update_single_move(res);

                match state {
                    OwnedArraySenderResult::Continue(sender) => {
                        let s = Self::PingerMAC { crc, sender };
                        (HandleResponse::Ping(s).into(), Some(res)).into()
                    }
                    OwnedArraySenderResult::Done => {
                        let s = Self::MyMAC {
                            crc,
                            sender: OwnedArraySender::new(core.my_mac),
                        };
                        (HandleResponse::Ping(s).into(), Some(res)).into()
                    }
                }
            }

            Self::MyMAC { crc, sender } => {
                let (state, res) = sender.handle();
                let crc = crc.update_single_move(res);

                match state {
                    OwnedArraySenderResult::Continue(sender) => {
                        let s = Self::MyMAC { crc, sender };
                        (HandleResponse::Ping(s).into(), Some(res)).into()
                    }
                    OwnedArraySenderResult::Done => {
                        let s = Self::Crc {
                            crc: crc.finalize(),
                        };
                        (HandleResponse::Ping(s).into(), Some(res)).into()
                    }
                }
            }
            Self::Crc { crc } => (WaitForStart {}.into(), Some(crc)).into(),
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
