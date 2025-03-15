#[repr(align(1))]
#[derive(Debug, Default, PartialEq)]
pub struct SDOAbort {
    pub operation: u8,
    pub index: u16,
    pub abort_code: u16,
}
