use core::sync::atomic::{compiler_fence, Ordering};

pub struct RingBuffer<T, const S: usize> {
    buf: [Option<T>; S],
    read: usize,
    write: usize,
}

impl<T, const S: usize> RingBuffer<T, S> {
    pub fn new() -> Self {
        Self {
            buf: [const { None }; S],
            read: 0,
            write: 0,
        }
    }

    pub fn push(&mut self, data: T) -> Result<(), T> {
        compiler_fence(Ordering::SeqCst);
        let write = self.write;
        compiler_fence(Ordering::SeqCst);

        let read = self.read;
        let next_write = Self::next(write); //write.overflowing_add(1).0;

        if next_write == read {
            return Err(data);
        }

        self.buf[write] = Some(data);

        compiler_fence(Ordering::SeqCst);
        self.write = next_write;
        compiler_fence(Ordering::SeqCst);

        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        compiler_fence(Ordering::SeqCst);
        let read = self.read;
        compiler_fence(Ordering::SeqCst);

        let write = self.write;

        if read == write {
            return None;
        }

        let data = self.buf[self.read].take();
        let read = Self::next(read); // read.overflowing_add(1).0;

        compiler_fence(Ordering::SeqCst);
        self.read = read;
        compiler_fence(Ordering::SeqCst);

        data
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
