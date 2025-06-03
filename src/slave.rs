mod request;
use replace_with::replace_with_or_abort;
pub use request::*;

mod response;
pub use response::*;

use crate::SINGLE_START_BYTE;

pub struct SlaveHandle {
    state: BusState,
    in_sync: bool,
}

#[derive(PartialEq, Debug)]
pub enum BusState {
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
            let (s, r) = s.rx(data, &mut self.in_sync);
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
    fn rx(self, data: u8, in_sync: &mut bool) -> (Self, Option<u8>) {
        match self {
            //
            // In the idle state, we essentially wait for the start byte.
            // If we receive anything other than the start byte, we might
            // be out of sync with the bus and disable the `in_sync` flag
            //
            Self::Idle => (
                if data == SINGLE_START_BYTE {
                    Self::Request(RequestState::Command)
                } else {
                    *in_sync = false;
                    Self::Idle
                },
                None,
            ),

            //
            // If we are an the request handling state, forward all
            // bytes to the request state RX handler to proceed
            //
            Self::Request(r) => r.rx(data),

            //
            // All other states are NOPs in the sense that a RX is not handled,
            // but the `in_sync` flag will be disabled, as a received byte in
            // these states is not expected and thus not allowed and would hint
            // at a bus timing or conformance violation.
            //
            x => {
                *in_sync = false;
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
        slave::{BusState, RequestState},
        SINGLE_START_BYTE,
    };

    /// Tests that the slave correctly handles an incoming start byte
    #[test]
    fn rx_single_start_byte() {
        let mut tmp = false;
        let state = BusState::Idle.rx(SINGLE_START_BYTE, &mut tmp);
        assert_eq!(
            state.0,
            BusState::Request(RequestState::Command),
            "Single-Command start byte"
        )
    }
}
