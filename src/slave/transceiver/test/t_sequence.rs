use crate::{
    slave::transceiver::test::{
        new_transceiver_in_sync, test_rx_crc_no_response, test_rx_no_response,
    },
    CMD_NOP, CMD_SYNC, PROTOCOL_VERSION_1, START_BYTE, SYNC_SEQUENCE,
};

#[test]
fn sequence_handling() {
    // Create 2 sequence numbers, one is the correct
    // one and the other for testing that the reaction
    // is correct
    for sequence_no in 0..4 {
        for test_no in 0..4 {
            new_transceiver_in_sync!(t);

            t.sequence_no = sequence_no;
            test_rx_no_response!(t, START_BYTE);
            test_rx_no_response!(t, CMD_NOP | ((test_no + 1) & 0b11) << 6);
            test_rx_crc_no_response!(t);

            // If the test number matches with the expected sequence
            // number, we expect no sync loss, otherwise we shuold loose
            // sync
            if test_no == sequence_no {
                assert!(t.in_sync, "Sync is lost with correct sequence {}", test_no);
            } else {
                assert!(
                    !t.in_sync,
                    "Sync is not lost after sequence violation {}",
                    test_no
                )
            }
        }
    }
}

#[test]
fn sync_sequence_no() {
    for sequence_no in 0..4 {
        new_transceiver_in_sync!(t);
        t.in_sync = false;

        test_rx_no_response!(t, START_BYTE);
        test_rx_no_response!(t, CMD_SYNC | sequence_no << 6);

        for b in SYNC_SEQUENCE {
            test_rx_no_response!(t, b);
        }

        test_rx_no_response!(t, PROTOCOL_VERSION_1);
        test_rx_crc_no_response!(t);

        assert_eq!(t.sequence_no, sequence_no);
    }
}
