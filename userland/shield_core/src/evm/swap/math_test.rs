/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::unwrap_used, clippy::integer_division)]

use super::{amount_out, impact_bps, mul_div};

#[test]
fn the_pool_formula_matches_uniswap_on_the_live_nox_reserves() {
    // The NOX/WETH pair's reserves as read from mainnet, and 1 ETH in.
    let (nox, weth) = (29_999_821_429_709_551_617_204_999u128, 36_333_445_222_658_394_009u128);
    let out = amount_out(1_000_000_000_000_000_000, weth, nox).unwrap();
    // The same formula in exact integers, worked out outside Rust.
    assert_eq!(out, 801217927807250245893724);
    assert!(out < nox);
}

#[test]
fn products_past_128_bits_are_carried_and_results_past_it_refused() {
    assert_eq!(mul_div(u128::MAX, 2, 4), Some(u128::MAX / 2));
    assert_eq!(mul_div(u128::MAX, u128::MAX, 1), None);
    assert_eq!(mul_div(1, 1, 0), None);
}

#[test]
fn impact_grows_with_size_and_is_zero_at_nothing() {
    let (r_in, r_out) = (36_000_000_000_000_000_000u128, 30_000_000_000_000_000_000_000_000u128);
    let small = 1_000_000_000_000_000u128;
    let large = 10_000_000_000_000_000_000u128;
    let i_small = impact_bps(small, amount_out(small, r_in, r_out).unwrap(), r_in, r_out).unwrap();
    let i_large = impact_bps(large, amount_out(large, r_in, r_out).unwrap(), r_in, r_out).unwrap();
    assert!(i_small <= 31, "a tiny trade pays about the 0.3% fee: {i_small}");
    assert!(i_large > 2_000, "ten ETH moves this pool by over 20%: {i_large}");
}
