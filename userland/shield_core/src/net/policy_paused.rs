//! Whether a pool's AmountPolicy is paused. Its guardian can stop every Shield withdrawal for up to
//! 14 days; a withdraw quoted and proved then would only be refused by the pool (IsPaused), so the
//! wallet asks first and says so. `paused()` is the policy's own, not the NOX token's.

use super::pool::{Pool, RPCS};
use super::rpc::call_over_tor;
use super::tor::{Purpose, Tor};
use crate::error::NetError;

const PAUSED: [u8; 4] = [0x5c, 0x97, 0x5a, 0xbb];

/// An ABI bool: zero or one, and nothing else.
pub fn bool_word(reply: &[u8]) -> Result<bool, NetError> {
    let w = reply.get(..32).filter(|_| reply.len() == 32).ok_or(NetError::ReplyShape)?;
    match (w[..31].iter().all(|b| *b == 0), w[31]) {
        (true, 0) => Ok(false),
        (true, 1) => Ok(true),
        _ => Err(NetError::ReplyShape),
    }
}

/// Whether `pool`'s policy is paused; a pool without a policy never is.
pub fn policy_paused(tor: &Tor, pool: &Pool) -> Result<bool, NetError> {
    let Some(policy) = pool.policy else { return Ok(false) };
    let mut last = NetError::Transport;
    for host in RPCS {
        match call_over_tor(tor, Purpose::Scan, host, policy, &PAUSED).and_then(|r| bool_word(&r)) {
            Ok(paused) => return Ok(paused),
            Err(e) => last = e,
        }
    }
    Err(last)
}
