//! A send's review: one batch read, then a refusal in a sentence or the transaction.

use super::super::calls::{balance, code, estimate, latest_block, nonce, priority_fee};
use super::super::network::Network;
use super::super::review_parts::{Checked, Order, Plan, Review};
use super::super::tx::Eip1559;
use super::super::{gas, review_checks};
use super::answers::answers;
use crate::error::NetError;
use crate::net::asset::Coin;
use crate::net::tor::Tor;

pub fn review(
    tor: &Tor,
    network: Network,
    from: &[u8; 20],
    order: &Order,
    sends: &[(u64, [u8; 32])],
) -> Result<Checked, NetError> {
    let chain = network.chain();
    if let Some(why) = review_checks::before_reading(&chain, from, order) {
        return Ok(Checked::Refused(why));
    }
    let plan = Plan::of(&chain, order);
    let mut calls = vec![
        balance(from),
        nonce(from),
        latest_block(),
        priority_fee(),
        code(&order.to),
        estimate(from, &plan.target, plan.value, &plan.data),
    ];
    calls.extend(super::super::review_token::calls(&chain, from, order));
    let answers = match answers(tor, &chain, order.coin, &calls)? {
        Ok(answers) => answers,
        Err(why) => return Ok(Checked::Refused(why)),
    };
    let at = |i: usize| answers.get(i).ok_or(NetError::ReplyShape);
    let ether = at(0)?.quantity()?;
    let pending = u64::try_from(at(1)?.quantity()?).map_err(|_| NetError::ReplyShape)?;
    let Some(fees) = gas::fees(at(2)?, at(3)?)? else {
        return Ok(Checked::Refused("Network fees are far above normal. Try again later."));
    };
    let contract = !at(4)?.data()?.is_empty();
    let arrival = match super::super::review_token::arrival(&chain, order, &answers)? {
        Ok(arrival) => arrival,
        Err(why) => return Ok(Checked::Refused(why)),
    };
    let plain = order.coin == Coin::Eth && !contract;
    let Some(gas) = review_checks::gas_limit(at(5)?, plain) else {
        return Ok(Checked::Refused("The network refused this transfer when it was tried."));
    };
    let max_network_fee = u128::from(gas).checked_mul(fees.max_fee).ok_or(NetError::ReplyShape)?;
    if let Some(why) = review_checks::afford(order, ether, max_network_fee) {
        return Ok(Checked::Refused(why));
    }
    let waiting = super::super::held::floor(tor, &chain, sends, pending)?;
    let tx = Eip1559 {
        chain_id: chain.chain_id,
        nonce: nox_verified::fee::next_nonce(pending, waiting),
        max_priority_fee: fees.max_priority_fee,
        max_fee: fees.max_fee,
        gas,
        to: plan.target,
        value: plan.value,
        data: plan.data,
    };
    Ok(Checked::Ready(Review { tx, arrival, max_network_fee, to_is_contract: contract }))
}
