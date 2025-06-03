mod request;
use replace_with::replace_with_or_abort;
pub use request::*;

mod response;
pub use response::*;

use crate::{
    crc8::{CRC8Autosar, CRC},
    SINGLE_START_BYTE,
};

#[derive(Debug, Default)]
pub struct SlaveHandle {
    state: BusState,
    core: SlaveCore,
}

#[derive(Debug, Default)]
pub struct SlaveCore {
    in_sync: bool,
}

#[derive(PartialEq, Debug, Default)]
pub enum BusState {
    #[default]
    Idle,
    Request(RequestState),
    ResponseStart(ResponseState),
    Response(ResponseState),
}

impl SlaveHandle {
    /// Handle an incoming byte from the bus endpoint
    /// # Arguments
    /// * `data` - The byte of data to be handled
    /// # Returns
    /// A possible byte to be sent back
    pub fn rx(&mut self, data: u8) -> Option<u8> {
        let mut response = None;

        replace_with_or_abort(&mut self.state, |s| {
            let (s, r) = s.rx(data, &mut self.core);
            response = r;
            s
        });

        response
    }

    /// Check if the bus has some data read to be sent
    /// # Returns
    /// A possible byte to be sent
    pub fn tx(&mut self) -> Option<u8> {
        let mut response = None;

        replace_with_or_abort(&mut self.state, |s| {
            let (s, r) = s.tx();
            response = r;
            s
        });

        response
    }
}

impl BusState {
    fn rx(self, data: u8, core: &mut SlaveCore) -> (Self, Option<u8>) {
        match self {
            //
            // In the idle state, we essentially wait for the start byte.
            // If we receive anything other than the start byte, we might
            // be out of sync with the bus and disable the `in_sync` flag
            //
            Self::Idle => (
                if data == SINGLE_START_BYTE {
                    Self::Request(RequestState::Command(
                        CRC8Autosar::new().update_single_move(data),
                    ))
                } else {
                    core.in_sync = false;
                    Self::Idle
                },
                None,
            ),

            //
            // If we are an the request handling state, forward all
            // bytes to the request state RX handler to proceed
            //
            Self::Request(r) => r.rx(data, core),

            //
            // All other states are NOPs in the sense that a RX is not handled,
            // but the `in_sync` flag will be disabled, as a received byte in
            // these states is not expected and thus not allowed and would hint
            // at a bus timing or conformance violation.
            //
            x => {
                core.in_sync = false;
                (x, None)
            }
        }
    }

    fn tx(self) -> (Self, Option<u8>) {
        match self {
            x => (x, None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        crc8::{CRC8Autosar, CRC},
        slave::{BusState, RequestState, SlaveCore, SlaveHandle},
        SINGLE_START_BYTE,
    };

    /// Tests that the slave correctly handles an incoming start byte
    #[test]
    fn rx_single_start_byte() {
        let mut core = SlaveCore { in_sync: false };
        let state = BusState::Idle.rx(SINGLE_START_BYTE, &mut core);
        assert_eq!(
            state.0,
            BusState::Request(RequestState::Command(
                CRC8Autosar::new().update_single_move(SINGLE_START_BYTE)
            )),
            "Single-Command start byte"
        )
    }

    #[test]
    fn rx_nop() {
        let mut handle = SlaveHandle::default();
        let mut crc = CRC8Autosar::new();

        handle.rx(SINGLE_START_BYTE);
        crc.update_single(SINGLE_START_BYTE);

        handle.rx(0x00);
        crc.update_single(0x00);

        assert_eq!(
            handle.state,
            BusState::Request(RequestState::CRC(None, crc.clone().finalize()))
        );

        handle.rx(crc.finalize());

        assert_eq!(handle.state, BusState::Idle);
    }
}
