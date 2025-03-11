use crate::{
    crc8::CRC,
    slave::frame::{core::Core, state::State, Receiver, Response},
};

#[derive(Debug, Default, PartialEq)]
pub struct RX01Ping {}

impl Receiver for RX01Ping {
    fn rx(self, data: u8, core: &mut Core) -> Response {
        core.crc.update_single(data);

        State::WaitForCRC(None).into()
    }
}
