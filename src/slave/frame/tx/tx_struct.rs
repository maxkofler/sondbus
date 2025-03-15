/// A handler to send a structure
#[derive(Default, Debug, PartialEq)]
pub struct StructSender {
    pos: usize,
}

/// A result from the `handle()` function
/// to indicate how to proceed
pub enum StructSenderResult {
    /// Continue with this handler - it is not yet done
    Continue(StructSender),
    /// The handler is done and the data is sent
    Done(),
}

impl StructSender {
    /// Handles an iteration on this handler yielding a byte
    /// # Returns
    /// A result to either continue with this handler or that it
    /// is done and the byte to be transmitted
    pub fn tx<T>(self, structure: &T) -> (StructSenderResult, u8) {
        self.tx_raw(structure as *const T as *const u8, size_of::<T>())
    }

    fn tx_raw(mut self, ptr: *const u8, size: usize) -> (StructSenderResult, u8) {
        let res = unsafe { *ptr.byte_add(self.pos) };
        self.pos += 1;

        let state = if self.pos >= size {
            StructSenderResult::Done()
        } else {
            StructSenderResult::Continue(self)
        };

        (state, res)
    }
}
