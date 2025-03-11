use core::marker::PhantomData;

/// A handler to pull in a structure from the bus
#[derive(Debug, PartialEq)]
pub struct OwnedStructReceiver<T> {
    data: T,
    pos: usize,
}

/// The result from the `handle()` function to
/// dictate what to do next
pub enum OwnedStructReceiverResult<T> {
    /// Continue with this handler - it is not yet done
    Continue(OwnedStructReceiver<T>),
    /// This handler is done and the data is ready
    Done(T),
}

impl<T> OwnedStructReceiver<T> {
    /// Construct a new handler
    /// # Arguments
    /// * `data` - The data to populate
    pub fn new(data: T) -> Self {
        Self { data, pos: 0 }
    }

    /// Handles an incoming byte
    /// # Arguments
    /// * `data` - The byte to process
    pub fn rx(mut self, data: u8) -> OwnedStructReceiverResult<T> {
        let ptr: *mut u8 = &mut self.data as *mut T as *mut u8;
        unsafe { *ptr.byte_add(self.pos) = data };

        self.pos += 1;

        if self.pos >= size_of::<T>() {
            OwnedStructReceiverResult::Done(self.data)
        } else {
            OwnedStructReceiverResult::Continue(self)
        }
    }
}

/// A handler to pull in a structure from the bus
#[derive(Default, Debug)]
pub struct StructReceiver<T> {
    pos: usize,
    _data: PhantomData<T>,
}

/// The result from the `handle()` function to
/// dictate what to do next
pub enum StructReceiverResult<T> {
    /// Continue with this handler - it is not yet done
    Continue(StructReceiver<T>),
    /// This handler is done and the data is ready
    Done,
}

impl<T> StructReceiver<T> {
    /// Handles an incoming byte
    /// # Arguments
    /// * `data` - The byte to process
    pub fn rx(mut self, data: u8, structure: &mut T) -> StructReceiverResult<T> {
        let ptr: *mut u8 = structure as *mut T as *mut u8;
        unsafe { *ptr = data };

        self.pos += 1;

        if self.pos >= size_of::<T>() {
            StructReceiverResult::Done
        } else {
            StructReceiverResult::Continue(self)
        }
    }
}
