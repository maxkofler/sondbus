use crate::{
    crc8::{CRC8Autosar, CRC},
    impl_handler, impl_tx_noop,
    slave::frame::{
        response::Response01Ping, Core, HandleData, HandlerResponse, RXHandler, SlaveState,
        WaitForCRC,
    },
    SlaveCore,
};

/// Handler for the `Ping` frame type (0x01)
pub enum Handler01Ping {
    HandleMyAddress { crc: CRC8Autosar, index: u8 },
    HandlePinger1 { crc: CRC8Autosar },
    HandlePinger2 { crc: CRC8Autosar, mac: [u8; 1] },
    HandlePinger3 { crc: CRC8Autosar, mac: [u8; 2] },
    HandlePinger4 { crc: CRC8Autosar, mac: [u8; 3] },
    HandlePinger5 { crc: CRC8Autosar, mac: [u8; 4] },
    HandlePinger6 { crc: CRC8Autosar, mac: [u8; 5] },
    Skip { crc: CRC8Autosar, remainder: u8 },
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

            // Handle the 1st octet of the sender
            Self::HandlePinger1 { crc } => (
                HandleData::Ping(Self::HandlePinger2 {
                    crc: crc.update_single_move(data),
                    mac: [data],
                })
                .into(),
                None,
            )
                .into(),

            // Handle the 2nd octet of the sender
            Self::HandlePinger2 { crc, mac } => (
                HandleData::Ping(Self::HandlePinger3 {
                    crc: crc.update_single_move(data),
                    mac: [mac[0], data],
                })
                .into(),
                None,
            )
                .into(),

            // Handle the 3rd octet of the sender
            Self::HandlePinger3 { crc, mac } => (
                HandleData::Ping(Self::HandlePinger4 {
                    crc: crc.update_single_move(data),
                    mac: [mac[0], mac[1], data],
                })
                .into(),
                None,
            )
                .into(),

            // Handle the 4th octet of the sender
            Self::HandlePinger4 { crc, mac } => (
                HandleData::Ping(Self::HandlePinger5 {
                    crc: crc.update_single_move(data),
                    mac: [mac[0], mac[1], mac[2], data],
                })
                .into(),
                None,
            )
                .into(),

            // Handle the 5th octet of the sender
            Self::HandlePinger5 { crc, mac } => (
                HandleData::Ping(Self::HandlePinger6 {
                    crc: crc.update_single_move(data),
                    mac: [mac[0], mac[1], mac[2], mac[3], data],
                })
                .into(),
                None,
            )
                .into(),

            // Handle the 6th octet of the sender
            Self::HandlePinger6 { crc, mac } => (
                WaitForCRC::new_with_response(
                    crc.update_single_move(data),
                    Response01Ping::new([mac[0], mac[1], mac[2], mac[3], mac[4], data]).into(),
                )
                .into(),
                None,
            )
                .into(),

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
                Self::HandlePinger1 { crc }
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
