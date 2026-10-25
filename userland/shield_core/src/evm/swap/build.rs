//! The transaction a reviewed swap signs: the exact approval it needs first,
//! or the router call with its minimum and deadline, estimated as the sender
//! so a swap that would revert is refused here, not paid for.

use super::checks::Plan;
use super::order::{Swap, SwapReview};
use super::route::Hop;
use super::venue::Venue;
use crate::error::NetError;
use crate::evm::calls::estimate;
use crate::evm::gas;
use crate::evm::reply::Answer;
use crate::evm::rpc::ask;
use crate::evm::sender::Sender;
use crate::evm::tx::Eip1559;
use crate::net::asset::Coin;

#[path = "build_parts.rs"]
mod parts;

/// How long a signed swap stays good: twenty minutes past the latest block.
const DEADLINE: u64 = 20 * 60;

pub(super) fn finish(
    sender: Sender,
    venue: &Venue,
    swap: &Swap,
    hops: &[Hop],
    first: &[Answer],
    plan: Plan,
    sends: &[(u64, [u8; 32])],
) -> Result<Result<SwapReview, &'static str>, NetError> {
    let Sender { tor, chain, from } = sender;
    let at = |i: usize| first.get(i).ok_or(NetError::ReplyShape);
    let ether = at(0)?.quantity()?;
    let pending = u64::try_from(at(1)?.quantity()?).map_err(|_| NetError::ReplyShape)?;
    let Some(fees) = gas::fees(at(2)?, at(3)?)? else {
        return Ok(Err("Network fees are far above normal. Try again later."));
    };
    if let Some(why) = parts::unfunded(swap, &plan, ether) {
        return Ok(Err(why));
    }
    let deadline = gas::timestamp(at(2)?)?.saturating_add(DEADLINE);
    let (target, value, data) = parts::call(venue, swap, &plan, hops, from, deadline)?;
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
    Ok(Ok(parts::review(tx, &plan, hops, max_network_fee)))
}
