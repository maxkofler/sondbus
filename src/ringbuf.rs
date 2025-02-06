use core::sync::atomic::{compiler_fence, Ordering};

/// A lock-free ring buffer
pub struct RingBuffer<T, const S: usize> {
    buf: [T; S],
    read: usize,
    write: usize,
}

impl<T, const S: usize> RingBuffer<T, S> {
    /// Create a new ring buffer
    /// # Arguments
    /// * `buf` - The underlying buffer of data to wrap
    pub fn new(buf: [T; S]) -> Self {
        Self {
            buf,
            read: 0,
            write: 0,
        }
    }

    /// Push a new element into the ring buffer
    /// # Arguments
    /// * `data` - The element to push
    /// # Returns
    /// Ok(), else the element that failed to insert
    pub fn push(&mut self, data: T) -> Result<(), T> {
        compiler_fence(Ordering::SeqCst);
        let write = self.write;
        compiler_fence(Ordering::SeqCst);

        let read = self.read;
        let next_write = Self::next(write); //write.overflowing_add(1).0;

        if next_write == read {
            return Err(data);
        }

        self.buf[write] = data;

        compiler_fence(Ordering::SeqCst);
        self.write = next_write;
        compiler_fence(Ordering::SeqCst);

        Ok(())
    }

    /// Retrieves one element from the ring buffer
    /// # Returns
    /// The element, if available, otherwise `None`
    pub fn pop(&mut self) -> Option<&T> {
        compiler_fence(Ordering::SeqCst);
        let read = self.read;
        compiler_fence(Ordering::SeqCst);

        let write = self.write;

        if read == write {
            return None;
        }

        let data = &self.buf[self.read];
        let read = Self::next(read);

        compiler_fence(Ordering::SeqCst);
        self.read = read;
        compiler_fence(Ordering::SeqCst);

        Some(data)
    }

    pub fn next(i: usize) -> usize {
        let new_i = i + 1;

        if new_i >= S {
            0
        } else {
            new_i
        }
    }
}
