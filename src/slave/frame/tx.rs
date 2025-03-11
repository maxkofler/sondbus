mod tx_01_ping;
pub use tx_01_ping::TX01Ping;

mod tx_struct;
pub use tx_struct::*;

use super::Receiver;

#[derive(Debug, PartialEq)]
pub enum TXType {
    Ping(TX01Ping),
}

impl Receiver for TXType {
    fn rx(self, data: u8, core: &mut super::core::Core) -> super::Response {
        match self {
            Self::Ping(v) => v.rx(data, core),
        }
    }
}
