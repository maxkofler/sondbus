//! Common utility functions used for testing the slave

use crate::slave::CallbackAction;

/// A callback that panics if called, informing that the callback
/// should never be called
pub fn rx_callback_panic(_: CallbackAction) -> bool {
    panic!("Callback was called when not allowed");
}
