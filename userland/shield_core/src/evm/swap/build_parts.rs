//! The two pieces of a swap's transaction that do not touch the network:
//! which call it makes, and the review it is shown in.

use super::super::calldata::{approve, eth_for_tokens, tokens_for};
use super::super::checks::Plan;
use super::super::order::{Swap, SwapReview};
use super::super::route::{path, Hop};
use super::super::venue::Venue;
use crate::error::NetError;
use crate::evm::tx::Eip1559;
use crate::net::asset::Coin;

/// The approval it needs first, or the router call with its minimum and
/// deadline: the target, the ETH it carries, and the calldata.
pub(super) fn call(
    venue: &Venue,
    swap: &Swap,
    plan: &Plan,
    hops: &[Hop],
    from: &[u8; 20],
    deadline: u64,
) -> Result<([u8; 20], u128, Vec<u8>), NetError> {
    let token_in = hops.first().map(|h| h.token_in).ok_or(NetError::ReplyShape)?;
    Ok(if plan.approval {
        (token_in, 0, approve(&venue.router, swap.amount))
    } else if swap.from == Coin::Eth {
        (venue.router, swap.amount, eth_for_tokens(plan.minimum, &path(hops), from, deadline))
    } else {
        let to_eth = swap.to == Coin::Eth;
        let data = tokens_for(to_eth, swap.amount, plan.minimum, &path(hops), from, deadline);
        (venue.router, 0, data)
    })
}

/// The review a person confirms, around the transaction it signs.
pub(super) fn review(tx: Eip1559, plan: &Plan, hops: &[Hop], max_network_fee: u128) -> SwapReview {
    SwapReview {
        tx,
        approval: plan.approval,
        expected: plan.expected,
        minimum: plan.minimum,
        token_fee: plan.token_fee,
        impact_bps: plan.impact_bps,
        max_network_fee,
        pairs: hops.iter().map(|h| h.pair.address).collect(),
    }
}

/// What the balance alone refuses, checked before the simulation: a node
/// refuses to simulate a swap the sender cannot pay for, and that refusal
/// would read as a price problem when it is an empty account.
pub(super) fn unfunded(swap: &Swap, plan: &Plan, ether: u128) -> Option<&'static str> {
    if ether == 0 {
        return Some("This account needs ETH to pay the network fee.");
    }
    (swap.from == Coin::Eth && !plan.approval && ether < swap.amount)
        .then_some("The ETH balance does not cover that.")
}
