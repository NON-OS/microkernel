// NONOS Operating System (AGPL-3.0-or-later)
//! What editing a typed amount must do: a backspace retraces, a second
//! point is ignored, and a typed zero is not an untouched field.

use super::amount_tests::{typed, ETH};
use crate::wallet::num::Amount;

#[test]
fn backspace_retraces_the_keys_that_were_pressed() {
    let a = typed("1.25<", ETH);
    assert_eq!(a.scaled(ETH), 1_200_000_000_000_000_000);
    let b = typed("1.25<<", ETH);
    assert_eq!(b.scaled(ETH), 1_000_000_000_000_000_000);
    // The next backspace takes the point itself, not another digit.
    let c = typed("1.25<<<", ETH);
    assert!(!c.point_started());
    assert_eq!(c.scaled(ETH), 1_000_000_000_000_000_000);
    // And only then the whole part.
    let d = typed("1.25<<<<", ETH);
    assert!(d.is_zero());
}

#[test]
fn backspace_on_an_empty_amount_is_harmless() {
    let a = typed("<<<", ETH);
    assert!(a.is_zero());
    assert!(!a.typed_anything());
}

#[test]
fn a_second_point_is_ignored_rather_than_counted() {
    assert_eq!(typed("1..5", ETH).scaled(ETH), typed("1.5", ETH).scaled(ETH));
}

/// But a keystroke that continues the reader's own typing appends as normal.
#[test]
fn typing_continues_a_figure_the_reader_typed() {
    assert_eq!(typed("12", ETH).scaled(ETH), 12_000_000_000_000_000_000);
}

/// Zero typed is not the same as nothing typed, and a screen that greys out its
/// sign button needs to tell them apart.
#[test]
fn a_typed_zero_is_distinguishable_from_an_untouched_field() {
    let untouched = Amount::new();
    assert!(!untouched.typed_anything());
    let zero = typed("0", ETH);
    assert!(zero.typed_anything());
    assert!(zero.is_zero());
}
