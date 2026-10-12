// NONOS Operating System (AGPL-3.0-or-later)
//! What the driver counts, and when it takes a device as gone.

use crate::nnet::{decode, Stats};
use crate::run::LOST_AFTER;

#[test]
fn errors_in_a_row_are_counted_and_a_moved_frame_clears_them() {
    let mut s = Stats::default();
    s.rx(Err(-5));
    s.tx(Err(-5));
    s.ticked(Err(-71));
    assert_eq!((s.failing, s.rx_errors, s.tx_errors), (3, 1, 1));
    s.rx(Ok(None));
    s.ticked(Ok(()));
    assert_eq!(s.failing, 3, "an empty poll and a good look move no frame");
    s.rx(Ok(Some(60)));
    assert_eq!(s.failing, 0);
    s.tx(Err(-5));
    s.tx(Ok(60));
    assert_eq!(s.failing, 0);
    assert_eq!(decode(&[0u8; 20]), None);
}

#[test]
fn a_device_whose_sends_fail_while_nothing_comes_in_is_given_up() {
    let mut s = Stats::default();
    for _ in 0..LOST_AFTER {
        s.tx(Err(-5));
        s.rx(Ok(None));
    }
    assert!(s.failing >= LOST_AFTER, "{} in a row", s.failing);
}
