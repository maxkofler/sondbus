use crate::{
    crc8::CRC,
    slave::frame::{core::Core, state::State, Receiver, Response},
};

#[derive(Debug, Default)]
pub struct RX00Sync {}

impl Receiver for RX00Sync {
    fn rx(self, data: u8, core: &mut Core) -> Response {
        core.crc.update_single(data);

        State::WaitForCRC.into()
    }
}
