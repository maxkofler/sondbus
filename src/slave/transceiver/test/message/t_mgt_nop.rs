use crate::slave::transceiver::{state::State, test::new_transceiver};

#[test]
fn management_nop() {
    new_transceiver!(t);

    t.t_handle_no_response(0);
    assert_eq!(t.state, State::CRC);

    t.t_handle_crc();
    assert_eq!(t.state, State::Idle);
    assert!(t.in_sync());
}
