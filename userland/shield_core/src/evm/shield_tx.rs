//! The transaction a reviewed deposit signs: the approval, or the deposit itself with its value,
//! each estimated as the sender, with the nonce past every send still waiting.

use super::calls::estimate;
use super::gas::Fees;
use super::network::Chain;
use super::rpc::ask;
use super::shield::{ShieldChecked, ShieldOrder, Shielding};
use super::tx::Eip1559;
use crate::error::NetError;
use crate::net::tor::Tor;

#[allow(clippy::too_many_arguments)]
pub(super) fn finish(
    tor: &Tor,
    chain: &Chain,
    from: &[u8; 20],
    order: &ShieldOrder,
    approval: bool,
    ether: u128,
    pending: u64,
    fees: Fees,
    sends: &[(u64, [u8; 32])],
) -> Result<ShieldChecked, NetError> {
    let value = if order.token.is_none() { order.amount } else { 0 };
    let (target, data) = match (approval, order.token) {
        (true, Some(token)) => (token, order.approve_data),
        _ => (order.pool, order.deposit_data),
    };
    let value = if approval { 0 } else { value };
    let answers = ask(tor, chain, &[estimate(from, &target, value, data)])?;
    let Some(gas) =
        answers.first().and_then(|a| a.quantity().ok()).and_then(|e| super::gas::limit(e, false))
    else {
        return Ok(ShieldChecked::Refused(if approval {
            "The token refused the approval when it was tried."
        } else {
            order.refused_as
        }));
    };
    let max_network_fee = u128::from(gas).checked_mul(fees.max_fee).ok_or(NetError::ReplyShape)?;
    if !nox_verified::fee::covers(ether, value, max_network_fee) {
        return Ok(ShieldChecked::Refused(
            "The ETH balance does not cover this and the most the network fee can be.",
        ));
    }
    let waiting = super::held::floor(tor, chain, sends, pending)?;
    let tx = Eip1559 {
        chain_id: chain.chain_id,
        nonce: nox_verified::fee::next_nonce(pending, waiting),
        max_priority_fee: fees.max_priority_fee,
        max_fee: fees.max_fee,
        gas,
        to: target,
        value,
        data: data.to_vec(),
    };
    Ok(ShieldChecked::Ready(Box::new(Shielding { tx, approval, max_network_fee })))
}
