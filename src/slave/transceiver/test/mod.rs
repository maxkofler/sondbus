mod message;

/// Creates a new transceiver named `$name` that is already in sync and
/// the sequence counter at 0b11, allowing the next command to use 0b00 for ease.
macro_rules! new_transceiver {
    ($name:ident, $scratchpad_size:expr, $physical_addr:expr) => {
        let mut __scratchpad = [0u8; $scratchpad_size];
        let mut $name = crate::slave::transceiver::Transceiver::new_in_sync_sc0(
            &mut __scratchpad,
            $physical_addr,
            |_| Err(()),
        );
    };
    ($name:ident, $scratchpad_size:expr) => {
        new_transceiver!($name, $scratchpad_size, [0, 0, 0, 0, 0, 0])
    };
    ($name:ident) => {
        new_transceiver!($name, 0)
    };
}

pub(super) use new_transceiver;
