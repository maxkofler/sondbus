use crate::{
    frameaction::{FrameAction, UnframedResponse},
    Bus, FrameDataHandler,
};

/// A cyclic request frame requesting cyclic data
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CyclicRequest {
    addr: Option<u8>,
    pos: u8,
    bytes_before: u16,
    bytes_budget: u8,
    bytes_after: u16,
}

impl CyclicRequest {
    pub fn new(addr: Option<u8>) -> Self {
        Self {
            addr,
            ..Default::default()
        }
    }
}
impl FrameDataHandler for CyclicRequest {
    fn handle(mut self, data: u8) -> Self {
        match self.addr {
            Some(addr) => {
                if self.pos < addr {
                    self.bytes_before += data as u16 + 1;
                } else if self.pos == addr {
                    self.bytes_budget = data;
                } else {
                    self.bytes_after += data as u16 + 1;
                }
            }
            None => self.bytes_before += data as u16,
        }

        self.pos += 1;

        self
    }

    fn commit(self, _bus: &mut dyn Bus) -> FrameAction {
        if self.bytes_before == 0 && self.bytes_budget == 0 && self.bytes_after == 0 {
            FrameAction::None
        } else {
            UnframedResponse::new(self.bytes_before, self.bytes_budget, self.bytes_after)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        frameaction::{FrameAction, UnframedResponse},
        frametype::{FrameDataHandler, FrameType},
        Bus,
    };

    use super::CyclicRequest;

    #[derive(Default)]
    pub struct DummyBus {}
    impl Bus for DummyBus {}

    /// Checks that a cyclic request data gets parsed correctly
    #[test]
    fn handle() {
        // Put the slave into a cyclic request with address `2`
        let mut ty = FrameType::CyclicRequest(CyclicRequest::new(Some(2)));

        // The data to receive:
        // - Master: 1 byte
        // - Another slave: 2 bytes
        // - We: 3 bytes
        // - Another slave: 4 bytes
        // - Another slave: 5 bytes
        let data = [1, 2, 3, 4, 5];

        for b in data {
            ty = ty.handle(b);
        }

        let res = ty.commit(&mut DummyBus::default());

        // Check that our request is correctly turned into an unframed response
        // We must add <slave count> bytes to leading and following for the crc they send
        assert_eq!(
            res,
            UnframedResponse::new(5, 3, 11),
            "Cyclic request frames are not parsed correctly"
        )
    }

    /// Checks that a cyclic request data gets parsed correctly
    /// if no leading bytes are to be expected
    #[test]
    fn handle_no_leading() {
        // Put the slave into a cyclic request with address `2`
        let mut ty = FrameType::CyclicRequest(CyclicRequest::new(Some(2)));

        // The data to receive:
        // - Master: 0 bytes
        // - Another slave: 0 bytes
        // - We: 3 bytes
        // - Another slave: 4 bytes
        // - Another slave: 5 bytes
        let data = [0, 0, 3, 4, 5];

        for b in data {
            ty = ty.handle(b);
        }

        let res = ty.commit(&mut DummyBus::default());

        // Check that our request is correctly turned into an unframed response
        // We must add <slave count> bytes to leading and following for the crc they send
        assert_eq!(
            res,
            UnframedResponse::new(2, 3, 11),
            "Cyclic request frames are not parsed correctly"
        )
    }

    /// Checks that a cyclic request data gets parsed correctly
    /// if no leading bytes are to be expected
    #[test]
    fn handle_no_budget() {
        // Put the slave into a cyclic request with address `2`
        let mut ty = FrameType::CyclicRequest(CyclicRequest::new(Some(2)));

        // The data to receive:
        // - Master: 1 bytes
        // - Another slave: 2 bytes
        // - We: 0 bytes
        // - Another slave: 4 bytes
        // - Another slave: 5 bytes
        let data = [1, 2, 0, 4, 5];

        for b in data {
            ty = ty.handle(b);
        }

        let res = ty.commit(&mut DummyBus::default());

        // Check that our request is correctly turned into an unframed response
        // We must add <slave count> bytes to leading and following for the crc they send
        assert_eq!(
            res,
            UnframedResponse::new(5, 0, 11),
            "Cyclic request frames are not parsed correctly"
        )
    }

    /// Checks that a cyclic request data gets parsed correctly
    /// if no leading bytes are to be expected
    #[test]
    fn handle_no_following() {
        // Put the slave into a cyclic request with address `2`
        let mut ty = FrameType::CyclicRequest(CyclicRequest::new(Some(2)));

        // The data to receive:
        // - Master: 1 bytes
        // - Another slave: 2 bytes
        // - We: 0 bytes
        // - Another slave: 0 bytes
        // - Another slave: 0 bytes
        let data = [1, 2, 3, 0, 0];

        for b in data {
            ty = ty.handle(b);
        }

        let res = ty.commit(&mut DummyBus::default());

        // Check that our request is correctly turned into an unframed response
        // We must add <slave count> bytes to leading and following for the crc they send
        assert_eq!(
            res,
            UnframedResponse::new(5, 3, 2),
            "Cyclic request frames are not parsed correctly"
        )
    }

    /// Checks that a cyclic request data gets parsed correctly
    /// if no leading bytes are to be expected
    #[test]
    fn handle_no_data() {
        // Put the slave into a cyclic request with address `2`
        let ty = FrameType::CyclicRequest(CyclicRequest::new(Some(2)));

        let res = ty.commit(&mut DummyBus::default());

        // Check that our request is correctly turned into no response,
        // since there was no data
        assert_eq!(
            res,
            FrameAction::None,
            "Cyclic request frames are not parsed correctly"
        )
    }
}
