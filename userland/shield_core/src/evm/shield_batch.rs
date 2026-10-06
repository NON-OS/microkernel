//! Several deposits from the public account under one review: each estimated as the sender with the
//! same fees, on consecutive nonces past every send still waiting, and the ETH for the amounts and
//! the most all their network fees can be checked as one sum before anything is signed.

use super::calls::{balance, call, estimate, latest_block, nonce, priority_fee};
use super::network::Network;
use super::nox::{with_address, BALANCE_OF};
use super::rpc::ask;
pub use super::shield_batch_txs::{Batch, BatchChecked, Deposit};
use crate::error::NetError;
use crate::net::tor::Tor;

const ALLOWANCE: [u8; 4] = [0xdd, 0x62, 0xed, 0x3e];

/// Review `deposits` into `pool` from `from`, of `token` when one is named, for `total` of it.
pub fn review_batch(
    tor: &Tor,
    from: &[u8; 20],
    (pool, token, total): (&[u8; 20], Option<[u8; 20]>, u128),
    deposits: &[Deposit],
    sends: &[(u64, [u8; 32])],
) -> Result<BatchChecked, NetError> {
    let chain = Network::Sepolia.chain();
    let mut calls = vec![balance(from), nonce(from), latest_block(), priority_fee()];
    if let Some(token) = token {
        let mut asked = with_address(ALLOWANCE, from);
        asked.extend_from_slice(&[0u8; 12]);
        asked.extend_from_slice(pool);
        calls.push(call(None, &token, &asked));
        calls.push(call(None, &token, &with_address(BALANCE_OF, from)));
    }
    let answers = ask(tor, &chain, &calls)?;
    let at = |i: usize| answers.get(i).ok_or(NetError::ReplyShape);
    let ether = at(0)?.quantity()?;
    let pending = u64::try_from(at(1)?.quantity()?).map_err(|_| NetError::ReplyShape)?;
    let Some(fees) = super::gas::fees(at(2)?, at(3)?)? else {
        return Ok(BatchChecked::Refused("Network fees are far above normal. Try again later."));
    };
    if token.is_some() && at(5)?.word()? < total {
        return Ok(BatchChecked::Refused("The NOX balance does not cover that."));
    }
    if token.is_some() && at(4)?.word()? < total {
        return Ok(BatchChecked::Approve);
    }
    let estimates: Vec<_> =
        deposits.iter().map(|d| estimate(from, pool, d.value, d.data)).collect();
    let gas = ask(tor, &chain, &estimates)?
        .iter()
        .map(|a| a.quantity().ok().and_then(|e| super::gas::limit(e, false)))
        .collect::<Option<Vec<u64>>>();
    let Some(gas) = gas.filter(|g| g.len() == deposits.len()) else {
        return Ok(BatchChecked::Refused("The pool would not take these deposits now."));
    };
    let waiting = super::held::floor(tor, &chain, sends, pending)?;
    let first = nox_verified::fee::next_nonce(pending, waiting);
    super::shield_batch_txs::build(&chain, (pool, deposits, &gas), (first, fees), ether)
}
