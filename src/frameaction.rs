//! All the possible actions that follow a frame

mod unframed_response;
pub use unframed_response::*;

/// The actions that follow a frame
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FrameAction {
    /// No action and response
    None,
    /// An unframed response to a frame
    UnframedResponse(UnframedResponse),
}
