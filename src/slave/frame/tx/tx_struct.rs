use core::marker::PhantomData;

/// A handler to send a structure owned by the handler
#[derive(Debug, PartialEq)]
pub struct OwnedStructSender<T> {
    data: T,
    pos: usize,
}

/// A result from the `handle()` function
/// to indicate how to proceed
pub enum OwnedStructSenderResult<T> {
    /// Continue with this handler - it is not yet done
    Continue(OwnedStructSender<T>),
    /// The handler is done and the data is sent
    Done(),
}

impl<T> OwnedStructSender<T> {
    /// Construct a new sender that is ready to
    /// send the supplied data
    /// * `data` - The data to be sent
    pub fn new(data: T) -> Self {
        Self { data, pos: 0 }
    }

    /// Handles an iteration on this handler yielding a byte
    /// # Returns
    /// A result to either continue with this handler or that it
    /// is done and the byte to be transmitted
    pub fn tx(mut self) -> (OwnedStructSenderResult<T>, u8) {
        let ptr: *const u8 = &self.data as *const T as *const u8;
        let res = unsafe { *ptr.byte_add(self.pos) };
        self.pos += 1;

        let state = if self.pos >= size_of::<T>() {
            OwnedStructSenderResult::Done()
        } else {
            OwnedStructSenderResult::Continue(self)
        };

        (state, res)
    }
}

/// A handler to send a structure
#[derive(Default)]
pub struct StructSender<T> {
    pos: usize,
    _data: PhantomData<T>,
}

/// A result from the `handle()` function
/// to indicate how to proceed
pub enum StructSenderResult<T> {
    /// Continue with this handler - it is not yet done
    Continue(StructSender<T>),
    /// The handler is done and the data is sent
    Done(),
}

impl<T> StructSender<T> {
    /// Handles an iteration on this handler yielding a byte
    /// # Returns
    /// A result to either continue with this handler or that it
    /// is done and the byte to be transmitted
    pub fn tx(mut self, structure: &T) -> (StructSenderResult<T>, u8) {
        let ptr: *const u8 = structure as *const T as *const u8;
        let res = unsafe { *ptr.byte_add(self.pos) };
        self.pos += 1;

        let state = if self.pos >= size_of::<T>() {
            StructSenderResult::Done()
        } else {
            StructSenderResult::Continue(self)
        };

        (state, res)
    }
}
