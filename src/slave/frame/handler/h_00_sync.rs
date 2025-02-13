use crate::{
    crc8::{CRC8Autosar, CRC},
    slave::{
        frame::{HandleData, Handler, WaitForCRC, WaitForStart},
        SlaveCore,
    },
    SYNC_SEQUENCE,
};

/// Handler for the `Sync` frame type (0x00)
pub struct Handler00Sync {
    pos: u8,
    crc: CRC8Autosar,
}

impl Handler00Sync {
    /// Create a new instance of the handler
    /// # Arguments
    /// * `crc` - The CRC over the received bytes
    pub fn new(crc: CRC8Autosar) -> Self {
        Self { crc, pos: 0 }
    }
}

impl Handler for Handler00Sync {
    fn rx(mut self, data: u8, core: &mut SlaveCore) -> crate::slave::frame::HandlerResponse {
        core.in_sync = false;

        if data == SYNC_SEQUENCE[self.pos as usize] {
            self.crc.update_single(data);
            self.pos += 1;

            if self.pos >= SYNC_SEQUENCE.len() as u8 {
                core.in_sync = true;

                (WaitForCRC::new(self.crc).into(), None).into()
            } else {
                (HandleData::Sync(self).into(), None).into()
            }
        } else {
            (WaitForStart {}.into(), None).into()
        }
    }

    fn tx(self, _core: &mut SlaveCore) -> crate::slave::frame::HandlerResponse {
        (HandleData::Sync(self).into(), None).into()
    }
}
