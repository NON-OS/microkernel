//! The transaction a reviewed swap signs: the exact approval it needs first,
//! or the router call with its minimum and deadline, estimated as the sender
//! so a swap that would revert is refused here, not paid for.

use super::calldata::{approve, eth_for_tokens, tokens_for};
use super::checks::Plan;
use super::order::{Swap, SwapReview};
use super::route::{path, Hop};
use super::venue::Venue;
use crate::error::NetError;
use crate::evm::calls::estimate;
use crate::evm::gas;
use crate::evm::network::Chain;
use crate::evm::reply::Answer;
use crate::evm::rpc::ask;
use crate::evm::tx::Eip1559;
use crate::net::asset::Coin;
use crate::net::tor::Tor;

/// How long a signed swap stays good: twenty minutes past the latest block.
const DEADLINE: u64 = 20 * 60;

#[allow(clippy::too_many_arguments)]
pub(super) fn finish(
    tor: &Tor,
    chain: &Chain,
    venue: &Venue,
    from: &[u8; 20],
    swap: &Swap,
    hops: &[Hop],
    first: &[Answer],
    plan: Plan,
    sends: &[(u64, [u8; 32])],
) -> Result<Result<SwapReview, &'static str>, NetError> {
    let at = |i: usize| first.get(i).ok_or(NetError::ReplyShape);
    let ether = at(0)?.quantity()?;
    let pending = u64::try_from(at(1)?.quantity()?).map_err(|_| NetError::ReplyShape)?;
    let Some(fees) = gas::fees(at(2)?, at(3)?)? else {
        return Ok(Err("Network fees are far above normal. Try again later."));
    };
    // The balance is checked before the simulation: a node refuses to
    // simulate a swap the sender cannot pay for, and that refusal would read
    // as a price problem when it is an empty account.
    if ether == 0 {
        return Ok(Err("This account needs ETH to pay the network fee."));
    }
    if swap.from == Coin::Eth && !plan.approval && ether < swap.amount {
        return Ok(Err("The ETH balance does not cover that."));
    }
    let deadline = gas::timestamp(at(2)?)?.saturating_add(DEADLINE);
    let token_in = hops.first().map(|h| h.token_in).ok_or(NetError::ReplyShape)?;
    let (target, value, data) = if plan.approval {
        (token_in, 0, approve(&venue.router, swap.amount))
    } else if swap.from == Coin::Eth {
        (venue.router, swap.amount, eth_for_tokens(plan.minimum, &path(hops), from, deadline))
    } else {
        let to_eth = swap.to == Coin::Eth;
        let data = tokens_for(to_eth, swap.amount, plan.minimum, &path(hops), from, deadline);
        (venue.router, 0, data)
    };
    let answers = ask(tor, chain, &[estimate(from, &target, value, &data)])?;
    let Some(gas) =
        answers.first().and_then(|a| a.quantity().ok()).and_then(|e| gas::limit(e, false))
    else {
        return Ok(Err(if plan.approval {
            "The token refused the approval when it was tried."
        } else {
            "The swap would not go through now. Swap less or allow more slippage."
        }));
    };
    let max_network_fee = u128::from(gas).checked_mul(fees.max_fee).ok_or(NetError::ReplyShape)?;
    let spend = if swap.from == Coin::Eth && !plan.approval { swap.amount } else { 0 };
    if !nox_verified::fee::covers(ether, spend, max_network_fee) {
        return Ok(Err("The ETH balance does not cover this and the most the network fee can be."));
    }
    let waiting = crate::evm::held::floor(tor, chain, sends, pending)?;
    let tx = Eip1559 {
        chain_id: chain.chain_id,
        nonce: nox_verified::fee::next_nonce(pending, waiting),
        max_priority_fee: fees.max_priority_fee,
        max_fee: fees.max_fee,
        gas,
        to: target,
        value,
        data,
    };
    Ok(Ok(SwapReview {
        tx,
        approval: plan.approval,
        expected: plan.expected,
        minimum: plan.minimum,
        token_fee: plan.token_fee,
        impact_bps: plan.impact_bps,
        max_network_fee,
        pairs: hops.iter().map(|h| h.pair.address).collect(),
    }))
}
