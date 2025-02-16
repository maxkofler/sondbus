/// A handler to send an array
pub struct OwnedArraySender<const S: usize> {
    data: [u8; S],
    pos: usize,
}

/// A result from the `handle()` function
/// to indicate how to proceed
pub enum OwnedArraySenderResult<const S: usize> {
    /// Continue with this handler - it is not yet done
    Continue(OwnedArraySender<S>),
    /// The handler is done and the data is sent
    Done,
}

impl<const S: usize> OwnedArraySender<S> {
    /// Construct a new sender that is ready to
    /// send the supplied data
    /// * `data` - The data to be sent
    pub fn new(data: [u8; S]) -> Self {
        Self { data, pos: 0 }
    }

    /// Handles an iteration on this handler yielding a byte
    /// # Returns
    /// A result to either continue with this handler or that it
    /// is done and the byte to be transmitted
    pub fn handle(mut self) -> (OwnedArraySenderResult<S>, u8) {
        let res = self.data[self.pos];
        self.pos += 1;

        let state = if self.pos >= S {
            OwnedArraySenderResult::Done
        } else {
            OwnedArraySenderResult::Continue(self)
        };

        (state, res)
    }
}
