//! What `owner` could take back, read from every RPC that answers, and agreed.

use super::{two_args, BETA_REFUNDABLE, CLAIMABLE};
use crate::error::NetError;
use crate::net::rpc::calls_over_tor;
use crate::net::tor::{Purpose, Tor};

/// The credited payout and beta refund `owner` could take for `asset`. Disagreeing RPCs refuse.
pub fn read_recoverable(
    tor: &Tor,
    hosts: &[&str],
    pool: &str,
    owner: &[u8; 20],
    asset: u64,
) -> Result<(u128, u128), NetError> {
    let calls = [two_args(&CLAIMABLE, asset, owner), two_args(&BETA_REFUNDABLE, asset, owner)];
    let mut agreed: Option<(u128, u128)> = None;
    let mut last = NetError::Transport;
    for host in hosts {
        match calls_over_tor(tor, Purpose::Deposit, host, pool, &calls).and_then(|w| pair(&w)) {
            Ok(now) => match agreed {
                None => agreed = Some(now),
                Some(prior) if prior == now => {}
                Some(_) => return Err(NetError::ReplyShape),
            },
            Err(e) => last = e,
        }
    }
    agreed.ok_or(last)
}

fn pair(words: &[Vec<u8>]) -> Result<(u128, u128), NetError> {
    let [a, b] = words else { return Err(NetError::ReplyShape) };
    Ok((uint(a)?, uint(b)?))
}

/// An ABI uint256 as u128, refusing anything that is not one word.
fn uint(word: &[u8]) -> Result<u128, NetError> {
    let word: &[u8; 32] = word.try_into().map_err(|_| NetError::ReplyShape)?;
    let (high, low) = word.split_at(16);
    if high.iter().any(|b| *b != 0) {
        return Ok(u128::MAX);
    }
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(low);
    Ok(u128::from_be_bytes(bytes))
}
