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

pub enum BatchChecked {
    Ready(Batch),
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
        let why = "The ETH balance does not cover these and the most their network fees can be.";
        return Ok(BatchChecked::Refused(why));
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

#[cfg(test)]
#[path = "shield_batch_test.rs"]
mod shield_batch_test;
