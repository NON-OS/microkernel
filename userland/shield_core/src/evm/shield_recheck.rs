//! A reviewed batch of ETH deposits asked about again at confirm, before anything is signed: the
//! balance, the fees and each deposit's gas now. The held transactions go out unchanged or not at
//! all; a deposit is never shrunk to make room for gas the review did not leave.

use super::calls::{balance, estimate, latest_block, priority_fee};
use super::network::Network;
use super::rpc::ask;
pub use super::shield_recheck_fit::{fit, Recheck};
use super::tx::Eip1559;
use crate::error::NetError;
use crate::net::tor::Tor;

/// Whether `txs`, as reviewed, from `from`, still fit the chain as it is now.
pub fn recheck_batch(tor: &Tor, from: &[u8; 20], txs: &[Eip1559]) -> Result<Recheck, NetError> {
    let chain = Network::Sepolia.chain();
    let answers = ask(tor, &chain, &[balance(from), latest_block(), priority_fee()])?;
    let at = |i: usize| answers.get(i).ok_or(NetError::ReplyShape);
    let ether = at(0)?.quantity()?;
    let Some(fees) = super::gas::fees(at(1)?, at(2)?)? else {
        return Ok(Recheck::Moved);
    };
    let estimates: Vec<_> = txs.iter().map(|t| estimate(from, &t.to, t.value, &t.data)).collect();
    let limits = ask(tor, &chain, &estimates)?
        .iter()
        .map(|a| a.quantity().ok().and_then(|e| super::gas::limit(e, false)))
        .collect::<Option<Vec<u64>>>();
    let Some(limits) = limits else { return Ok(Recheck::Moved) };
    Ok(fit(ether, fees.max_fee, &limits, txs))
}
