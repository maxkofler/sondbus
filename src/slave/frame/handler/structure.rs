use core::marker::PhantomData;

use crate::crc8::{CRC8Autosar, CRC};

/// A handler to pull in a structure from the bus
pub struct OwnedStructHandler<T> {
    crc: CRC8Autosar,
    data: T,
    pos: usize,
}

/// The result from the `handle()` function to
/// dictate what to do next
pub enum OwnedStructHandlerResult<T> {
    /// Continue with this handler - it is not yet done
    Continue(OwnedStructHandler<T>),
    /// This handler is done and the data is ready
    Done(T, CRC8Autosar),
}

impl<T> OwnedStructHandler<T> {
    /// Construct a new handler
    /// # Arguments
    /// * `crc` - The CRC to populate
    /// * `data` - The data to populate
    pub fn new(crc: CRC8Autosar, data: T) -> Self {
        Self { data, crc, pos: 0 }
    }

    /// Handles an incoming byte
    /// # Arguments
    /// * `data` - The byte to process
    pub fn handle(mut self, data: u8) -> OwnedStructHandlerResult<T> {
        self.crc.update_single(data);
        let ptr: *mut u8 = &mut self.data as *mut T as *mut u8;
        unsafe { *ptr = data };

        self.pos += 1;

        if self.pos >= size_of::<T>() {
            OwnedStructHandlerResult::Done(self.data, self.crc)
        } else {
            OwnedStructHandlerResult::Continue(self)
        }
    }
}

/// A handler to pull in a structure from the bus
#[derive(Default)]
pub struct StructHandler<T> {
    crc: CRC8Autosar,
    pos: usize,
    _data: PhantomData<T>,
}

/// The result from the `handle()` function to
/// dictate what to do next
pub enum StructHandlerResult<T> {
    /// Continue with this handler - it is not yet done
    Continue(StructHandler<T>),
    /// This handler is done and the data is ready
    Done(CRC8Autosar),
}

impl<T> StructHandler<T> {
    /// Handles an incoming byte
    /// # Arguments
    /// * `data` - The byte to process
    pub fn handle(mut self, data: u8, structure: &mut T) -> StructHandlerResult<T> {
        self.crc.update_single(data);
        let ptr: *mut u8 = structure as *mut T as *mut u8;
        unsafe { *ptr = data };

        self.pos += 1;

        if self.pos >= size_of::<T>() {
            StructHandlerResult::Done(self.crc)
        } else {
            StructHandlerResult::Continue(self)
        }
    }
}
