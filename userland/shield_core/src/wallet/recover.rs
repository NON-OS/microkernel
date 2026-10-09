//! Getting money back the pool holds: `claim` for a failed unshield payout, and `betaRefund`
//! for a beta deposit, the only exit after a wind-down. Both are sent from the user's own wallet.
//! `commitRoot()` is here too, since a note is provable only once a published root contains it.

use crate::error::NetError;
use crate::net::rpc::calls_over_tor;
use crate::net::tor::{Purpose, Tor};

const CLAIM: [u8; 4] = [0x88, 0x1a, 0x1c, 0xe0];
const CLAIMABLE: [u8; 4] = [0x88, 0xd8, 0xb2, 0xa7];
const BETA_REFUND: [u8; 4] = [0xf9, 0xa8, 0xcd, 0x1d];
const BETA_REFUNDABLE: [u8; 4] = [0xc9, 0x03, 0x67, 0xc5];
const COMMIT_ROOT: [u8; 4] = [0xd3, 0x43, 0x53, 0xc9];

/// `claim(assetId, to)`: collect a credited payout. Only the credited address may send it.
pub fn claim_calldata(asset: u64, to: &[u8; 20]) -> Vec<u8> {
    two_args(&CLAIM, asset, to)
}

/// `betaRefund(assetId, depositor)`: funds always go to the depositor, whoever sends it.
pub fn beta_refund_calldata(asset: u64, depositor: &[u8; 20]) -> Vec<u8> {
    two_args(&BETA_REFUND, asset, depositor)
}

/// `commitRoot()`: publish a root containing every leaf inserted so far.
pub fn commit_root_calldata() -> Vec<u8> {
    COMMIT_ROOT.to_vec()
}

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

fn two_args(selector: &[u8; 4], asset: u64, addr: &[u8; 20]) -> Vec<u8> {
    let mut out = selector.to_vec();
    let mut asset_word = [0u8; 32];
    asset_word.split_at_mut(24).1.copy_from_slice(&asset.to_be_bytes());
    let mut addr_word = [0u8; 32];
    addr_word.split_at_mut(12).1.copy_from_slice(addr);
    out.extend_from_slice(&asset_word);
    out.extend_from_slice(&addr_word);
    out
}
