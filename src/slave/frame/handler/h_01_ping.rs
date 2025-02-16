use crate::{
    crc8::{CRC8Autosar, CRC},
    impl_handler, impl_tx_noop,
    slave::frame::{
        response::Response01Ping, Core, HandleData, HandlerResponse, RXHandler, SlaveState,
        WaitForCRC,
    },
    SlaveCore,
};

use super::{array::ArrayHandlerResult, ArrayHandler};

/// Handler for the `Ping` frame type (0x01)
pub enum Handler01Ping {
    HandleMyAddress {
        crc: CRC8Autosar,
        index: u8,
    },
    HandlePinger {
        crc: CRC8Autosar,
        array: ArrayHandler<6>,
    },
    Skip {
        crc: CRC8Autosar,
        remainder: u8,
    },
}

impl Handler01Ping {
    /// Create a new instance of the ping handler
    /// # Arguments
    /// * `crc` - The CRC over the received bytes
    pub fn new(crc: CRC8Autosar) -> Self {
        Self::HandleMyAddress { crc, index: 0 }
    }
}

impl RXHandler for Handler01Ping {
    fn rx(self, data: u8, core: &mut SlaveCore) -> HandlerResponse {
        match self {
            Self::HandleMyAddress { crc, index } => {
                Self::handle_wait_for_my_address(data, core, crc, index)
            }

            Self::HandlePinger { crc, array } => {
                let crc = crc.update_single_move(data);
                match array.handle(data) {
                    ArrayHandlerResult::Continue(array) => {
                        let s = Self::HandlePinger { crc, array };
                        (HandleData::Ping(s).into(), None).into()
                    }
                    ArrayHandlerResult::Done(mac) => (
                        WaitForCRC::new_with_response(crc, Response01Ping::new(mac).into()).into(),
                        None,
                    )
                        .into(),
                }
            }

            Self::Skip { crc, remainder } => Self::handle_skip(data, crc, remainder),
        }
    }
}

impl Handler01Ping {
    fn handle_wait_for_my_address(
        data: u8,
        core: &mut SlaveCore,
        crc: CRC8Autosar,
        index: u8,
    ) -> HandlerResponse {
        let crc = crc.update_single_move(data);

        let s = if core.my_mac[index as usize] == data {
            if index >= 5 {
                Self::HandlePinger {
                    crc,
                    array: ArrayHandler::default(),
                }
            } else {
                Self::HandleMyAddress {
                    crc,
                    index: index + 1,
                }
            }
        } else {
            Self::Skip {
                crc,
                remainder: 12 - index,
            }
        };

        (HandleData::Ping(s).into(), None).into()
    }

    fn handle_skip(data: u8, crc: CRC8Autosar, remainder: u8) -> HandlerResponse {
        let remainder = remainder - 1;
        let crc = crc.update_single_move(data);

        if remainder == 0 {
            (WaitForCRC::new(crc).into(), None).into()
        } else {
            (HandleData::Ping(Self::Skip { crc, remainder }).into(), None).into()
        }
    }
}

impl From<Handler01Ping> for SlaveState {
    fn from(value: Handler01Ping) -> Self {
        SlaveState::HandleData(Core(HandleData::Ping(value)))
    }
}

impl_tx_noop!(Handler01Ping);
impl_handler!(Handler01Ping);
