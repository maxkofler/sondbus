#[derive(Debug)]
pub enum ReadObjectError {
    /// The requested object id is not
    /// available in the object dictionary
    UnknownObject(u16),
}

impl ReadObjectError {
    pub fn abort_code(&self) -> u16 {
        match self {
            Self::UnknownObject(_) => 0x10,
        }
    }
}

#[derive(Debug)]
pub enum WriteObjectError {
    /// The requested object id is not
    /// available in the object dictionary
    UnknownObject(u16),

    /// The size of the submitted array is invalid
    /// and cannot be processed
    InvalidSize { expected: usize, received: usize },
}

/// The callbacks for `sondbus` to use for interacting with
/// the hosting application.
///
/// All callbacks **must** perform adequately to not block
/// the bus communication.
pub struct Callbacks<'a> {
    /// Callback for reading an object in a cyclic and acyclic manner
    ///
    /// # Arguments
    /// * `index`: The index of the object to read
    /// * `data`: A buffer to write data to
    /// # Returns
    /// A reference to the data of the object to be read
    pub read_object: &'a dyn Fn(u16, &mut [u8]) -> Result<usize, ReadObjectError>,

    /// Callback for writing an object in a cyclic and acyclic manner
    ///
    /// # Arguments
    /// * `index`: The index of the object to write
    /// * `data`: The data to be written
    /// # Returns
    /// A reference to the data of the object to be read
    pub write_object: &'a mut dyn FnMut(u16, &[u8]) -> Result<(), WriteObjectError>,
}
