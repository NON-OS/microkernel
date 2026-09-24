// NONOS Operating System (AGPL-3.0-or-later)
//! What typing an amount of money must do.
//!
//! These run over the shipping `Amount`, because the figure this produces is the
//! figure that gets signed. The cases that matter are the ones where a wallet
//! quietly changes what you asked for: a fraction that rounds, a backspace that
//! does not retrace, a shortcut figure that a keystroke appends to.

use crate::wallet::num::Amount;

const ETH: u32 = 18;

fn typed(keys: &str, decimals: u32) -> Amount {
    let mut a = Amount::new();
    for c in keys.chars() {
        match c {
            '.' => a.start_point(),
            '<' => a.backspace(),
            d => a.digit(d as u8 - b'0', decimals),
        }
    }
    a
}

#[test]
fn a_whole_amount_scales_by_the_full_precision() {
    assert_eq!(typed("1", ETH).scaled(ETH), 1_000_000_000_000_000_000);
    assert_eq!(typed("25", ETH).scaled(ETH), 25_000_000_000_000_000_000);
}

/// The figure the old send screen could not express. This is the whole reason
/// the shared keypad exists.
#[test]
fn a_fraction_below_a_thousandth_is_expressible() {
    assert_eq!(typed("0.0005", ETH).scaled(ETH), 500_000_000_000_000);
}

#[test]
fn a_long_fraction_is_kept_exactly() {
    assert_eq!(typed("0.12345", ETH).scaled(ETH), 123_450_000_000_000_000);
}

/// One wei: the smallest thing the chain can move, and a wallet that cannot say
/// it cannot claim to spend an exact balance.
#[test]
fn a_single_wei_is_expressible() {
    let a = typed("0.000000000000000001", ETH);
    assert_eq!(a.scaled(ETH), 1);
}

/// Nothing may be accepted past the token's precision, because a place the chain
/// cannot count would multiply the amount by ten on the way out.
#[test]
fn digits_past_the_precision_are_refused_not_absorbed() {
    let capped = typed("0.0000000000000000019", ETH);
    assert_eq!(capped.places(), ETH);
    assert_eq!(capped.scaled(ETH), 1, "the refused digit must not shift the figure");
}

#[test]
fn precision_is_per_token() {
    // A six-decimal token: the same keys mean a different chain figure.
    assert_eq!(typed("1.5", 6).scaled(6), 1_500_000);
    assert_eq!(typed("1.5", ETH).scaled(ETH), 1_500_000_000_000_000_000);
}

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

/// A figure from a drag or a max shortcut is a whole amount. Typing over it must
/// start a new number rather than append a digit to it, which would multiply the
/// balance by ten with one keystroke.
#[test]
fn typing_over_a_shortcut_figure_replaces_it() {
    let mut a = Amount::new();
    a.set_scaled(7_000_000_000_000_000_000, ETH);
    assert_eq!(a.scaled(ETH), 7_000_000_000_000_000_000);
    a.digit(3, ETH);
    assert_eq!(a.scaled(ETH), 3_000_000_000_000_000_000, "keying must replace, not append");
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
