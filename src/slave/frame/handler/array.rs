/// A handler to handle an incoming array
pub struct ArrayHandler<const S: usize> {
    data: [u8; S],
    pos: usize,
}

/// A result from the `handle()` function
/// to indicate how to proceed
pub enum ArrayHandlerResult<const S: usize> {
    /// Continue with this handler - it is not yet done
    Continue(ArrayHandler<S>),
    /// The handler is done and the resulting data can be used
    Done([u8; S]),
}

impl<const S: usize> Default for ArrayHandler<S> {
    fn default() -> Self {
        Self {
            data: [0u8; S],
            pos: Default::default(),
        }
    }
}

impl<const S: usize> ArrayHandler<S> {
    /// Handles an incoming byte
    /// # Arguments
    /// * `data` - The byte of data to handle
    /// # Returns
    /// A result to either continue with this handler or that it is done
    pub fn handle(mut self, data: u8) -> ArrayHandlerResult<S> {
        self.data[self.pos] = data;
        self.pos += 1;

        if self.pos >= S {
            ArrayHandlerResult::Done(self.data)
        } else {
            ArrayHandlerResult::Continue(self)
        }
    }
}
