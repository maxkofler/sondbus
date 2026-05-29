mod message;

/// Creates a new transceiver named `$name` that is already in sync and
/// the sequence counter at 0b11, allowing the next command to use 0b00 for ease.
macro_rules! new_transceiver {
    ($name:ident, $scratchpad_size:expr) => {
        let mut __scratchpad = [0u8; $scratchpad_size];
        let mut $name = Transceiver::new(&mut __scratchpad, [0, 0, 0, 0, 0, 0], |_| Err(()));
        $name.in_sync = true;
        $name.sequence_no = 0b11;
    };
    ($name:ident) => {
        new_transceiver!($name, 0)
    };
}

pub(super) use new_transceiver;
