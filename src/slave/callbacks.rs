/// A data type that can be sent over the bus
#[derive(Debug)]
pub enum DataType {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),
}

#[derive(Debug)]
pub enum ReadObjectError {
    /// The requested object id is not
    /// available in the object dictionary
    UnknownObject(u16),
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
    /// # Returns
    /// A reference to the data of the object to be read
    pub read_object: &'a dyn Fn(u16) -> Result<DataType, ReadObjectError>,

    /// Callback for writing an object in a cyclic and acyclic manner
    ///
    /// # Arguments
    /// * `index`: The index of the object to write
    /// * `data`: The data to be written
    /// # Returns
    /// A reference to the data of the object to be read
    pub write_object: &'a mut dyn FnMut(u16, &[u8]) -> Result<(), WriteObjectError>,
}
