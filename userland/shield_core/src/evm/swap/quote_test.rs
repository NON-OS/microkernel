/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::arithmetic_side_effects)]

use super::{quote, NoxState};
use crate::evm::swap::math::amount_out;
use crate::evm::swap::route::route;
use crate::evm::swap::MAINNET;
use crate::evm::Network;
use crate::net::asset::Coin;

const NOX_R: u128 = 29_999_821_429_709_551_617_204_999;
const WETH_R: u128 = 36_333_445_222_658_394_009;
const E18: u128 = 1_000_000_000_000_000_000;

fn state(fee_sale: Option<(u128, u128, u128)>) -> NoxState {
    NoxState {
        token: MAINNET.nox_weth.token0,
        buy_bps: 0,
        sell_bps: 300,
        burn_share_bps: 1_000,
        exempt: false,
        fee_sale,
    }
}

#[test]
fn a_sale_pays_the_sell_fee_before_the_pool_sees_it() {
    let hops = route(&Network::Mainnet.chain(), &MAINNET, Coin::Nox, Coin::Eth).unwrap();
    let q = quote(&hops, &[(NOX_R, WETH_R)], 1_000 * E18, Some(state(None))).unwrap();
    assert_eq!(q.token_fee, 30 * E18);
    assert_eq!(q.out, amount_out(970 * E18, NOX_R, WETH_R).unwrap());
}

#[test]
fn a_fee_sale_that_fires_first_lowers_what_the_seller_gets() {
    let hops = route(&Network::Mainnet.chain(), &MAINNET, Coin::Nox, Coin::Eth).unwrap();
    let quiet = quote(&hops, &[(NOX_R, WETH_R)], 1_000 * E18, Some(state(None))).unwrap();
    // 49,990 NOX held and a 27 NOX cut takes it past the 50,000 threshold.
    let firing = Some((50_000 * E18, 500_000 * E18, 49_990 * E18));
    let fired = quote(&hops, &[(NOX_R, WETH_R)], 1_000 * E18, Some(state(firing))).unwrap();
    assert!(fired.out < quiet.out, "the token's own sale moved the price first");
    let below = Some((50_000 * E18, 500_000 * E18, 30_000 * E18));
    let held = quote(&hops, &[(NOX_R, WETH_R)], 1_000 * E18, Some(state(below))).unwrap();
    assert_eq!(held.out, quiet.out, "below the threshold nothing fires");
}

#[test]
fn nox_to_usdc_goes_through_weth_in_two_hops() {
    let hops = route(&Network::Mainnet.chain(), &MAINNET, Coin::Nox, Coin::Usdc).unwrap();
    assert_eq!(hops.len(), 2);
    assert_eq!(hops[0].token_out, MAINNET.weth);
    assert_eq!(hops[1].token_in, MAINNET.weth);
    let usdc = (30_000_000 * 1_000_000u128, 10_000 * E18);
    let q = quote(&hops, &[(NOX_R, WETH_R), (usdc.1, usdc.0)], 1_000 * E18, Some(state(None)));
    assert!(q.unwrap().out > 0);
}
