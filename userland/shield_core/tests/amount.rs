//! Amounts on the launch pool, which counts NOX in note units of 10^9 base
//! units and ETH in wei: one NOX is 10^9 units, one ETH is 10^18, and a note
//! of either holds at most p - 2 of them.

#[path = "amount/coins.rs"]
mod coins;

use nox_shield_core::ffi::{format_amount, max_note_amount, parse_amount};
use nox_shield_core::net::asset::Coin::Nox;
use nox_shield_core::notes::MAX_VALUE;

fn format(units: u64) -> String {
    format_amount(Nox, units)
}

fn parse(text: &str) -> Result<u64, nox_shield_core::error::WalletError> {
    parse_amount(Nox, text.into())
}

/// One NOX in note units.
const NOX: u64 = 1_000_000_000;
const HALF: u64 = 500_000_000;

#[test]
fn an_amount_round_trips_through_its_text() {
    for units in [0u64, 1, NOX - 1, NOX, 2 * NOX + HALF, MAX_VALUE] {
        let text = format(units);
        assert_eq!(parse(&text).expect("our own text parses"), units, "{text}");
    }
}

#[test]
fn the_decimal_point_follows_the_pool_scale() {
    assert_eq!(format(NOX), "1");
    assert_eq!(format(NOX + HALF), "1.5");
    assert_eq!(format(1), "0.000000001", "one note unit is 10^9 base units");
    assert_eq!(parse("1").expect("one token"), NOX);
}

#[test]
fn a_note_holds_at_most_max_value_units() {
    assert_eq!(max_note_amount(Nox), "18446744069.414584319");
    assert_eq!(parse("18446744069.414584319").expect("the ceiling"), MAX_VALUE);
    assert!(parse("18446744069.41458432").is_err(), "one unit over");
    assert!(parse("0.0000000001").is_err(), "less than one note unit");
}

#[test]
fn junk_is_refused_rather_than_read_as_a_number() {
    for text in ["", ".", "1.2.3", "-1", "1e9", "0x10", "1.0000000000000000001", "abc"] {
        assert!(parse(text).is_err(), "{text} was accepted");
    }
}

#[test]
fn a_comma_is_read_as_a_decimal_point() {
    assert_eq!(parse("1,5").expect("a comma is a decimal point"), NOX + HALF);
}
