/// A handler to pull in a structure from the bus
#[derive(Default, Debug, PartialEq)]
pub struct StructReceiver {
    pos: usize,
}

/// The result from the `handle()` function to
/// dictate what to do next
pub enum StructReceiverResult {
    /// Continue with this handler - it is not yet done
    Continue(StructReceiver),
    /// This handler is done and the data is ready
    Done,
}

impl StructReceiver {
    /// Handles an incoming byte
    /// # Arguments
    /// * `data` - The byte to process
    pub fn rx<T>(self, data: u8, structure: &mut T) -> StructReceiverResult {
        self.rx_raw(data, structure as *mut T as *mut u8, size_of::<T>())
    }

    fn rx_raw(mut self, data: u8, ptr: *mut u8, size: usize) -> StructReceiverResult {
        unsafe { *ptr.byte_add(self.pos) = data };

        self.pos += 1;

        if self.pos >= size {
            StructReceiverResult::Done
        } else {
            StructReceiverResult::Continue(self)
        }
    }
}
