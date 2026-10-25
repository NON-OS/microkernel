//! The checks that turn a reading into a refusal, each in one sentence a
//! screen can show as it is.

#[path = "review_checks_arrival.rs"]
mod arrival;

pub(super) use arrival::{nox_arrival, token_agrees, usdc_arrival};

use super::network::Chain;
use super::reply::Answer;
use super::review_parts::Order;
use crate::net::asset::Coin;

/// The answers every send reads, in order, before the token's.
pub(super) const COMMON: usize = 6;

/// Refusals that need no reading: nothing, nowhere, or somewhere that keeps it.
pub(super) fn before_reading(
    chain: &Chain,
    from: &[u8; 20],
    order: &Order,
) -> Option<&'static str> {
    if order.amount == 0 {
        return Some("Enter an amount above zero.");
    }
    if order.to == [0u8; 20] {
        return Some("That is the zero address. Anything sent there is lost.");
    }
    if &order.to == from {
        return Some("That is this account's own address.");
    }
    if order.to == chain.nox_token || order.to == chain.usdc_token {
        return Some("That is a token contract itself. Anything sent there is lost.");
    }
    None
}

/// The gas limit, or none when the estimate says the transfer would fail.
pub(super) fn gas_limit(estimate: &Answer, plain: bool) -> Option<u64> {
    if plain {
        return super::gas::limit(0, true);
    }
    super::gas::limit(estimate.quantity().ok()?, false)
}

/// Whether the ether on hand covers what the send takes of it.
pub(super) fn afford(order: &Order, ether: u128, max_fee: u128) -> Option<&'static str> {
    match order.coin {
        Coin::Eth if !nox_verified::fee::covers(ether, order.amount, max_fee) => {
            Some("The ETH balance does not cover the amount and the most the network fee can be.")
        }
        Coin::Nox | Coin::Usdc if ether < max_fee => {
            Some("This account needs ETH to pay the network fee.")
        }
        _ => None,
    }
}

#[cfg(kani)]
#[path = "review_checks_kani.rs"]
mod review_checks_kani;
