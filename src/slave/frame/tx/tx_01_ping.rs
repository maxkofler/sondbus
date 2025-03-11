use crate::{impl_receiver_nop, slave::frame::state::State};

use super::TXType;

#[derive(Debug)]
pub struct TX01Ping {}

impl From<TX01Ping> for State {
    fn from(value: TX01Ping) -> Self {
        Self::HandleTX(TXType::Ping(value))
    }
}

impl_receiver_nop!(TX01Ping);
