/// A handler to send an array
#[derive(Default, Debug, PartialEq)]
pub struct ArraySender {
    pos: usize,
    len: usize,
}

/// A result from the `handle()` function
/// to indicate how to proceed
pub enum ArraySenderResult {
    /// Continue with this handler - it is not yet done
    Continue(ArraySender),
    /// The handler is done and the data is sent
    Done(),
}

impl ArraySender {
    pub fn new_with_len(len: usize) -> Self {
        Self { pos: 0, len }
    }

    /// Handles an iteration on this handler yielding a byte
    /// # Returns
    /// A result to either continue with this handler or that it
    /// is done and the byte to be transmitted
    pub fn tx(mut self, data: &[u8]) -> (ArraySenderResult, u8) {
        let ptr: *const u8 = data.as_ptr();
        let res = unsafe { *ptr.byte_add(self.pos) };
        self.pos += 1;

        let len = if self.len == 0 { data.len() } else { self.len };

        let state = if self.pos >= len {
            ArraySenderResult::Done()
        } else {
            ArraySenderResult::Continue(self)
        };

        (state, res)
    }
}
