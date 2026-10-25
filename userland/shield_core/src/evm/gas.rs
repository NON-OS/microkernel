//! The network fee a send offers, from the latest block.
//! EIP-1559: the tip is the RPC's suggestion held to 5 gwei, so a lying RPC
//! cannot talk a send into overpaying. The ceiling is twice the latest base fee
//! plus the tip, and a ceiling past 1,000 gwei is refused, whatever the RPC said.

use super::reply::Answer;
use crate::error::NetError;
use crate::net::rpc::{after, quoted};

#[cfg(kani)]
use nox_verified::fee::{CEILING, MAX_TIP};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Fees {
    pub max_priority_fee: u128,
    pub max_fee: u128,
}

/// The fees from the answers to the latest block and the suggested tip, or
/// none when the ceiling would pass `CEILING`.
pub(super) fn fees(block: &Answer, tip: &Answer) -> Result<Option<Fees>, NetError> {
    Ok(offer(base_fee(block.result()?)?, tip.quantity()?))
}

/// The offer for a base fee and a suggested tip, from the verified kernel,
/// with the bounds proved in Lean.
pub(super) fn offer(base: u128, suggested: u128) -> Option<Fees> {
    nox_verified::fee::offer(base, suggested)
        .map(|(tip, max_fee)| Fees { max_priority_fee: tip, max_fee })
}

/// The latest block's time, in seconds, for a swap's deadline.
pub(super) fn timestamp(block: &Answer) -> Result<u64, NetError> {
    let text = block.result()?;
    let hex = after(text, "\"timestamp\"").and_then(quoted).ok_or(NetError::ReplyShape)?;
    let body = hex.strip_prefix("0x").ok_or(NetError::ReplyShape)?;
    u64::from_str_radix(body, 16).map_err(|_| NetError::ReplyShape)
}

pub(super) fn base_fee(block: &str) -> Result<u128, NetError> {
    let hex = after(block, "\"baseFeePerGas\"").and_then(quoted).ok_or(NetError::ReplyShape)?;
    let body = hex.strip_prefix("0x").ok_or(NetError::ReplyShape)?;
    u128::from_str_radix(body, 16).map_err(|_| NetError::ReplyShape)
}

/// The gas limit: 21,000 for ether to an address with no code, otherwise the
/// estimate with a quarter on top, since a token transfer's cost can move.
pub(super) fn limit(estimate: u128, plain_ether: bool) -> Option<u64> {
    if plain_ether {
        return Some(21_000);
    }
    let padded = estimate.checked_mul(5)?.checked_div(4)?;
    u64::try_from(padded).ok()
}

#[cfg(kani)]
#[path = "gas_kani.rs"]
mod gas_kani;
#[cfg(test)]
#[path = "gas_test.rs"]
mod tests;
