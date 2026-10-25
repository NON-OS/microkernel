// NONOS Operating System (AGPL-3.0-or-later)
//! What typing an amount of money must do.
//!
//! These run over the shipping `Amount`, because the figure this produces is the
//! figure that gets signed. The cases that matter are the ones where a wallet
//! quietly changes what you asked for: a fraction that rounds, a backspace that
//! does not retrace, a figure that wraps into a different, payable one.

use crate::wallet::num::Amount;

pub(crate) const ETH: u32 = 18;

pub(crate) fn typed(keys: &str, decimals: u32) -> Amount {
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
