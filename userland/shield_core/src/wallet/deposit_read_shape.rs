//! The words a pool's shape decides: a launch pool's scale and open deposits, a policy's fee.

use super::abi::{amount, call, flag};
use crate::error::NetError;
use crate::net::pool::Pool;
use crate::net::rpc::calls_over_tor;
use crate::net::tor::{Purpose, Tor};

/// `bps()` and `paused()` of an amount policy, which holds the deposit fee once a pool has one.
const POLICY_BPS: [u8; 4] = [0x68, 0x23, 0x73, 0x29];
const POLICY_PAUSED: [u8; 4] = [0x5c, 0x97, 0x5a, 0xbb];

/// The fee word and the policy's pause. A pool with no policy holds its own fee, the last word.
pub(super) fn fee_and_pause(
    tor: &Tor,
    host: &str,
    pool: &Pool,
    words: &mut Vec<[u8; 32]>,
) -> Result<([u8; 32], bool), NetError> {
    Ok(match pool.policy {
        Some(policy) => {
            let both = calls_over_tor(
                tor,
                Purpose::Deposit,
                host,
                policy,
                &[call(&POLICY_BPS, &[]), call(&POLICY_PAUSED, &[])],
            )?;
            let [bps, paused] = <[Vec<u8>; 2]>::try_from(both).map_err(|_| NetError::ReplyShape)?;
            let word =
                |w: Vec<u8>| <[u8; 32]>::try_from(w.as_slice()).map_err(|_| NetError::ReplyShape);
            (word(bps)?, flag(word(paused)?)?)
        }
        None => (words.pop().ok_or(NetError::ReplyShape)?, false),
    })
}

/// A launch pool's scale and open deposits, the two words past `base`. A format 5 pool has
/// neither: a scale of one, and no open deposits.
pub(super) fn launch_terms(
    words: &mut Vec<[u8; 32]>,
    base: usize,
    launch: bool,
) -> Result<(u128, bool), NetError> {
    Ok(if launch {
        let [scale, open] =
            <[[u8; 32]; 2]>::try_from(words.split_off(base)).map_err(|_| NetError::ReplyShape)?;
        (amount(scale), flag(open)?)
    } else {
        (1, false)
    })
}
