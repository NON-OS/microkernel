/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::integer_division
)]

use super::{outcome, Outcome, Rules};
use crate::evm::nox_revert::why;

const PLAIN: Rules = Rules {
    paused: false,
    blocked: false,
    fees: [0, 300, 0],
    pair_from: false,
    pair_to: false,
    exempt: false,
};
const NOX: u128 = 1_000_000_000_000_000_000;

#[test]
fn a_wallet_to_wallet_transfer_pays_the_transfer_fee() {
    assert_eq!(outcome(&PLAIN, NOX), Outcome::Arrives { amount: NOX, fee: 0, bps: 0 });
    let taxed = Rules { fees: [0, 300, 250], ..PLAIN };
    let fee = NOX / 40;
    assert_eq!(outcome(&taxed, NOX), Outcome::Arrives { amount: NOX - fee, fee, bps: 250 });
}

#[test]
fn a_transfer_into_a_pair_pays_the_sell_fee_and_an_exemption_pays_none() {
    let sell = Rules { pair_to: true, ..PLAIN };
    let fee = NOX * 3 / 100;
    assert_eq!(outcome(&sell, NOX), Outcome::Arrives { amount: NOX - fee, fee, bps: 300 });
    let exempt = Rules { exempt: true, ..sell };
    assert_eq!(outcome(&exempt, NOX), Outcome::Arrives { amount: NOX, fee: 0, bps: 0 });
}

#[test]
fn a_pause_or_the_blacklist_refuses_before_any_fee() {
    assert!(matches!(outcome(&Rules { paused: true, ..PLAIN }, NOX), Outcome::Refused(_)));
    assert!(matches!(outcome(&Rules { blocked: true, ..PLAIN }, NOX), Outcome::Refused(_)));
}

#[test]
fn a_fee_that_takes_everything_is_refused() {
    let all = Rules { fees: [0, 0, 10_000], ..PLAIN };
    assert!(matches!(outcome(&all, NOX), Outcome::Refused(_)));
}

#[test]
fn revert_data_names_the_reason() {
    // `cast calldata "Error(string)" g`, as the token reverts with it.
    let mut blocked = vec![0x08, 0xc3, 0x79, 0xa0];
    blocked.extend_from_slice(&[0u8; 31]);
    blocked.push(0x20);
    blocked.extend_from_slice(&[0u8; 31]);
    blocked.push(1);
    blocked.push(b'g');
    blocked.extend_from_slice(&[0u8; 31]);
    assert_eq!(why(&blocked), "The token blocks the sender or the recipient.");
    assert_eq!(why(&[0xd9, 0x3c, 0x06, 0x65]), "NOX transfers are paused on this network.");
    assert_eq!(why(&[]), "The token refused this transfer when it was tried.");
}
