/*
 * A test asserts by panicking, and a measurement prints what it read, so the
 * lints that forbid panicking and indexing are off here and nowhere else.
 */
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

//! Amounts on the launch pool, which counts NOX in note units of 10^9 base
//! units and ETH in wei: one NOX is 10^9 units, one ETH is 10^18, and a note
//! of either holds at most p - 2 of them.

use nox_shield_core::ffi::{amount_unit, format_amount, max_note_amount, parse_amount};
use nox_shield_core::net::asset::Coin::{self, Eth, Nox};
use nox_shield_core::net::pool::ACTIVE;
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

#[test]
fn every_coin_a_screen_names_is_held_by_the_active_pool() {
    // The shield holds ETH and NOX, and USDC lives in the public account only.
    for coin in [Eth, Nox] {
        assert!(ACTIVE.asset(coin).is_some(), "{coin:?} missing from the active pool");
    }
    assert!(ACTIVE.asset(Coin::Usdc).is_none());
    assert_eq!((amount_unit(Eth), amount_unit(Nox)), ("ETH".into(), "NOX".into()));
}

#[test]
fn eth_is_counted_in_wei() {
    const ETH: u64 = 1_000_000_000_000_000_000;
    let eth = |text: &str| parse_amount(Coin::Eth, text.into());
    assert_eq!(eth("1").expect("one ether"), ETH);
    assert_eq!(eth("0.000000000000000001").expect("one wei"), 1);
    assert_eq!(format_amount(Eth, 1_000_000_000_000_000), "0.001");
    assert_eq!(max_note_amount(Eth), "18.446744069414584319");
    assert!(eth("18.44674406941458432").is_err(), "one wei over a note");
    for units in [0u64, 1, ETH, MAX_VALUE] {
        assert_eq!(eth(&format_amount(Eth, units)).expect("round trip"), units);
    }
}
