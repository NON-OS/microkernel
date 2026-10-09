// NONOS Operating System (AGPL-3.0-or-later)
//! What the driver counts, and when it takes a device as gone.

use crate::nnet::{decode, Stats};

#[test]
fn errors_in_a_row_are_counted_and_a_success_clears_them() {
    let mut s = Stats::default();
    s.rx(Err(-5));
    s.tx(Err(-5));
    assert_eq!((s.failing, s.rx_errors, s.tx_errors), (2, 1, 1));
    s.rx(Ok(None));
    assert_eq!(s.failing, 0);
    assert_eq!(decode(&[0u8; 20]), None);
}
