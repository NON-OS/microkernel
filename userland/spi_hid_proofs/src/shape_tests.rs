// NONOS Operating System (AGPL-3.0-or-later)
//! An SPI report laid out as the shared poll reads an I2C frame.

use crate::shape::shape;

#[test]
fn a_numbered_report_carries_its_id_after_the_length() {
    let mut buf = [0u8; 16];
    assert_eq!(shape(5, &[1, 2, 3], &mut buf), Some(6));
    assert_eq!(buf[..6], [6, 0, 5, 1, 2, 3]);
}

#[test]
fn an_unnumbered_report_follows_the_length() {
    let mut buf = [0u8; 16];
    assert_eq!(shape(0, &[9, 8], &mut buf), Some(4));
    assert_eq!(buf[..4], [4, 0, 9, 8]);
}

#[test]
fn a_report_that_does_not_fit_is_dropped() {
    let mut buf = [0u8; 8];
    assert_eq!(shape(1, &[0; 5], &mut buf), Some(8));
    assert_eq!(shape(1, &[0; 6], &mut buf), None);
    assert_eq!(shape(0, &[0; 7], &mut buf), None);
}
