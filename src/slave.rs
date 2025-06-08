use replace_with::replace_with_or_abort;

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
            Self::Idle => (Self::Idle, None),
        }
    }

    /// Change the core's sync flag to false and go back to Idle
    /// # Arguments
    /// * `core` - The core to drop out of sync
    /// # Returns
    /// The new state
    fn sync_lost(core: &mut SlaveCore) -> Self {
        core.in_sync = false;
        Self::Idle
    }

    fn tx(self) -> (Self, Option<u8>) {
        match self {
            x => (x, None),
        }
    }
}

#[cfg(test)]
mod tests {}
