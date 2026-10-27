// NONOS Operating System (AGPL-3.0-or-later)
//! The DAC calibration's conversions, each a known answer worked from
//! rtw8822c.c's arithmetic.

use crate::phy::dack::codes::{code, compensation, left, magnitude};

#[test]
fn compensation_negates_all_but_zero() {
    assert_eq!(compensation(0), 0);
    assert_eq!(compensation(3), 0x3FD);
    assert_eq!(compensation(0x3FD), 3);
}

#[test]
fn magnitude_folds_the_negatives() {
    assert_eq!(magnitude(4), 4);
    assert_eq!(magnitude(0x1FF), 0x1FF);
    assert_eq!(magnitude(0x200), 0x200);
    assert_eq!(magnitude(0x3FC), 4);
}

#[test]
fn the_step_3_code_has_a_six_fifths_slope() {
    assert_eq!(code(0), 0x80);
    assert_eq!(code(0x3FB), 5 * 12 / 5 + 0x80, "-5 compensates to 5");
    assert_eq!(code(0x100), 0x7F_u32.wrapping_sub(0x100 * 12 / 5), "0x300 and up");
    assert_eq!(code(0x101), 0x2FF * 12 / 5 + 0x80, "just under 0x300");
}

#[test]
fn what_is_left_is_unbiased_then_folded() {
    assert_eq!(left(0x10), 0);
    assert_eq!(left(0x13), 3);
    assert_eq!(left(0x0D), 3, "0x400 - 3, folded");
    assert_eq!(left(0x3F0), 0x20);
}
