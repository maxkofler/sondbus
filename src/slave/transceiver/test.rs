use crate::slave::transceiver::state::State;

//mod mem_cmd;
mod t_cmd_mem_addressed;
mod t_cmd_mem_broadcast;
mod t_cmd_nop;
mod t_cmd_sync;
mod t_sequence;

/// Test that the supplied transceiver is in the correct state
macro_rules! test_state {
    ($t:expr, $s:expr) => {
        assert_eq!(
            $t.state.clone(),
            $s,
            "Expected state '{:?}', got '{:?}'",
            $s,
            &$t.state
        )
    };
}

/// Test that the supplied transceiver is in the correct state
macro_rules! test_sync {
    ($t:expr, $s:expr) => {
        assert_eq!($t.in_sync, $s, "Expected sync {}, got {}", $s, &$t.in_sync)
    };
}

/// Run an iteration of `handle()` and make sure there is
/// no response given by the transceiver
macro_rules! test_rx_no_response {
    ($t:expr, $v:expr) => {
        let r = $t.handle(Some($v));
        assert!(r.is_none(), "Expected no response, got 0x{:x?}", r.unwrap())
    };
}

/// Run an iteration of `handle()` and make sure there is
/// no response given by the transceiver
macro_rules! test_tx {
    ($t:expr, $tx: expr) => {
        let r = $t.handle(None);
        assert!(r.is_some(), "Expected response 0x{:x?}, got none", $tx);
        let r = r.unwrap();
        assert_eq!(r, $tx, "Expected TX of 0x{:x}, got 0x{:x}", $tx, r);
    };
    ($t:expr, $rx:expr, $tx: expr) => {
        let r = $t.handle(Some($rx));
        assert!(r.is_some(), "Expected response 0x{:x?}, got none", $tx);
        let r = r.unwrap();
        assert_eq!(r, $tx, "Expected TX of 0x{:x}, got 0x{:x}", $tx, r);
    };
}

/// Create a new transceiver instance that
/// is already initialized and in sync
macro_rules! new_transceiver_in_sync {
    ($t:ident) => {
        new_transceiver_in_sync!($t, [0u8; 6])
    };
    ($t: ident, $addr: expr) => {
        let mut scratchpad = [0u8; 0xf];
        let mut $t = crate::slave::transceiver::Transceiver::new(&mut scratchpad, $addr);
        $t.in_sync = true;
        $t.sequence_no = 0b11;
    };
}

macro_rules! test_rx_crc_no_response {
    ($t: expr) => {
        let crc = $crate::crc8::CRC::finalize(&$t.crc);
        crate::slave::transceiver::test::test_rx_no_response!($t, crc);
    };
}

macro_rules! test_consequence {
    ($t: expr, $v:expr) => {
        assert_eq!(
            $t.consequence, $v,
            "Expected consequence '{:?}', got '{:?}'",
            $v, $t.consequence
        );
    };
}

pub(crate) use {
    new_transceiver_in_sync, test_consequence, test_rx_crc_no_response, test_rx_no_response,
    test_state, test_sync, test_tx,
};

#[test]
fn test_new() {
    new_transceiver_in_sync!(t);
    assert_eq!(t.state, State::WaitForStart)
}

#[test]
fn test_start() {
    new_transceiver_in_sync!(t);
    let res = t.handle(Some(0x55));
    assert!(res.is_none());
    assert_eq!(t.state, State::WaitForCommand);
}
