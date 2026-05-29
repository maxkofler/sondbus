use crate::{
    slave::transceiver::{state::State, Transceiver},
    SYNC_SEQUENCE,
};

use super::super::new_transceiver;

#[test]
fn management_nop() {
    new_transceiver!(t);
    t.in_sync = false;

    t.t_handle_no_response(1);
    assert_eq!(t.state, State::ManagementSync);

    for b in SYNC_SEQUENCE {
        t.t_handle_no_response(b);
    }

    t.t_handle_crc();

    assert!(t.in_sync);
}
