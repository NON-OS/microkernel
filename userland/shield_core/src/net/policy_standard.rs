//! Whether an amount is a standard size, asked of the pool itself (M3, v1.4.17): on a pool with an
//! AmountPolicy, isStandard(assetId, units) decides, at confirm and again before proving, since a
//! new range applies at activateRange with no grace. A pool without a policy keeps its own sizes.

use super::asset::Asset;
use super::policy_paused::bool_word;
use super::policy_rules::{decode_rule, Rule};
use super::pool::{Pool, RPCS};
use super::rpc::call_over_tor;
use super::tor::{Purpose, Tor};
use crate::error::NetError;

const IS_STANDARD: [u8; 4] = [0x8b, 0xda, 0x98, 0x14];
const RULES: [u8; 4] = [0x19, 0xa7, 0x47, 0xfc];

fn word(v: u64) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[24..].copy_from_slice(&v.to_be_bytes());
    w
}

fn ask(tor: &Tor, policy: &str, data: &[u8]) -> Result<Vec<u8>, NetError> {
    let mut last = NetError::Transport;
    for host in RPCS {
        match call_over_tor(tor, Purpose::Scan, host, policy, data) {
            Ok(r) => return Ok(r),
            Err(e) => last = e,
        }
    }
    Err(last)
}

/// Whether `units` of `asset` is standard on `pool` now.
pub fn standard_now(tor: &Tor, pool: &Pool, asset: Asset, units: u64) -> Result<bool, NetError> {
    let Some(policy) = pool.policy else { return Ok(asset.allows(units)) };
    let data = [&IS_STANDARD[..], &word(asset.id), &word(units)].concat();
    bool_word(&ask(tor, policy, &data)?)
}

/// The policy's rule for `asset`, to show its range; none on a pool without a policy.
pub fn rule_now(tor: &Tor, pool: &Pool, asset: Asset) -> Result<Option<Rule>, NetError> {
    let Some(policy) = pool.policy else { return Ok(None) };
    let reply = ask(tor, policy, &[&RULES[..], &word(asset.id)].concat())?;
    decode_rule(&reply).map(Some).ok_or(NetError::ReplyShape)
}
