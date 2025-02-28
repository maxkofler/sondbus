/// A handler to handle an incoming, owned array
pub struct OwnedArrayHandler<const S: usize> {
    data: [u8; S],
    pos: usize,
}

/// A result from the `handle()` function
/// to indicate how to proceed
pub enum OwnedArrayHandlerResult<const S: usize> {
    /// Continue with this handler - it is not yet done
    Continue(OwnedArrayHandler<S>),
    /// The handler is done and the resulting data can be used
    Done([u8; S]),
}

impl<const S: usize> Default for OwnedArrayHandler<S> {
    fn default() -> Self {
        Self {
            data: [0u8; S],
            pos: Default::default(),
        }
    }
}

impl<const S: usize> OwnedArrayHandler<S> {
    /// Handles an incoming byte
    /// # Arguments
    /// * `data` - The byte of data to handle
    /// # Returns
    /// A result to either continue with this handler or that it is done
    pub fn handle(mut self, data: u8) -> OwnedArrayHandlerResult<S> {
        self.data[self.pos] = data;
        self.pos += 1;

        if self.pos >= S {
            OwnedArrayHandlerResult::Done(self.data)
        } else {
            OwnedArrayHandlerResult::Continue(self)
        }
    }
}
