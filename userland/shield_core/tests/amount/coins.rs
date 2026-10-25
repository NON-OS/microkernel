//! The coins a screen names, held by the active pool, and ETH counted in wei.

use nox_shield_core::ffi::{amount_unit, format_amount, max_note_amount, parse_amount};
use nox_shield_core::net::asset::Coin::{self, Eth, Nox};
use nox_shield_core::net::pool::ACTIVE;
use nox_shield_core::notes::MAX_VALUE;

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
