//! The transactions of a reviewed batch of deposits, and a single balance check that covers them.

use super::gas::Fees;
use super::network::Chain;
use super::tx::Eip1559;
use crate::error::NetError;

/// One deposit of a batch: the ETH it carries, zero for a token, and its call to the pool.
pub struct Deposit<'a> {
    pub value: u128,
    pub data: &'a [u8],
}

/// The transactions to sign in order, and the most their network fees can be together.
pub struct Batch {
    pub txs: Vec<Eip1559>,
    pub max_network_fee: u128,
}

/// What a public review says when ETH falls short of the values and the fees.
pub const BATCH_ETH_SHORT: &str =
    "The ETH balance does not cover these and the most their network fees can be.";

pub enum BatchChecked {
    Ready(Batch),
    /// The ETH it lacks for the values and the most the network fees can be.
    ShortOfEth(u128),
    /// The pool may not pull the whole amount yet. The exact approval comes first.
    Approve,
    Refused(&'static str),
}

pub(super) fn build(
    chain: &Chain,
    (pool, deposits, gas): (&[u8; 20], &[Deposit], &[u64]),
    (first, fees): (u64, Fees),
    ether: u128,
) -> Result<BatchChecked, NetError> {
    let wide = |g: &u64| u128::from(*g).checked_mul(fees.max_fee);
    let fee = gas.iter().try_fold(0u128, |sum, g| sum.checked_add(wide(g)?));
    let value = deposits.iter().try_fold(0u128, |sum, d| sum.checked_add(d.value));
    let (Some(max_network_fee), Some(value)) = (fee, value) else {
        return Err(NetError::ReplyShape);
    };
    if !nox_verified::fee::covers(ether, value, max_network_fee) {
        return Ok(BatchChecked::ShortOfEth(short(ether, value, max_network_fee)));
    }
    let mut txs = Vec::with_capacity(deposits.len());
    for (i, (deposit, gas)) in deposits.iter().zip(gas).enumerate() {
        let next = u64::try_from(i).ok().and_then(|i| first.checked_add(i));
        let nonce = next.ok_or(NetError::ReplyShape)?;
        txs.push(Eip1559 {
            chain_id: chain.chain_id,
            nonce,
            max_priority_fee: fees.max_priority_fee,
            max_fee: fees.max_fee,
            gas: *gas,
            to: *pool,
            value: deposit.value,
            data: deposit.data.to_vec(),
        });
    }
    Ok(BatchChecked::Ready(Batch { txs, max_network_fee }))
}

/// What `ether` lacks for `value` and `fee`.
pub fn short(ether: u128, value: u128, fee: u128) -> u128 {
    value.checked_add(fee).map_or(u128::MAX, |need| need.saturating_sub(ether))
}

#[cfg(test)]
#[path = "shield_batch_test.rs"]
mod shield_batch_test;
