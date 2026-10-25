// NONOS Operating System (AGPL-3.0-or-later)
//! An amount's limits: it saturates rather than wraps, clears to untouched,
//! ignores a non-digit, and is filled exactly by what is held.

use super::amount_tests::{typed, ETH};
use crate::wallet::num::Amount;

/// Saturating, never wrapping: an amount this wallet cannot sign must stay
/// unpayable rather than wrap into a different, payable figure.
#[test]
fn an_impossible_amount_saturates_instead_of_wrapping() {
    let mut a = Amount::new();
    for _ in 0..40 {
        a.digit(9, ETH);
    }
    assert_eq!(a.scaled(ETH), u128::MAX);
}

#[test]
fn clearing_returns_it_to_untouched() {
    let mut a = typed("1.5", ETH);
    a.clear();
    assert!(a.is_zero());
    assert!(!a.typed_anything());
    assert!(!a.point_started());
    assert_eq!(a.places(), 0);
}

/// Digits are the only thing the keypad takes. A stray code must not be folded
/// into the figure.
#[test]
fn a_non_digit_is_ignored() {
    let mut a = Amount::new();
    a.digit(9, ETH);
    a.digit(200, ETH);
    assert_eq!(a.raw(), 9);
}

/* "Use all" fills the field with what is held, exactly: scaled back it is
 * the same base units, and it reads as the figure a person would type. */
#[test]
fn a_held_amount_fills_the_field_exactly() {
    for (v, dp) in [
        (1_234_500_000_000_000_000u128, 18u32),
        (1, 18),
        (7_000_000_000_000_000_000, 18),
        (0, 18),
        (1_250_000, 6),
        (999_999, 6),
    ] {
        let a = Amount::of_units(v, dp);
        assert_eq!(a.scaled(dp), v, "{v} at {dp}");
    }
    let a = Amount::of_units(1_250_000, 6);
    assert_eq!((a.raw(), a.places(), a.point_started()), (125, 2, true), "reads 1.25");
    let whole = Amount::of_units(7_000_000, 6);
    assert_eq!((whole.raw(), whole.places(), whole.point_started()), (7, 0, false), "reads 7");
}
