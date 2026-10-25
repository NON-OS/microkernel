//! The policy's calls and replies as ABI words: the calls built, the schedule and bps read.

use super::{Schedule, SETTLEMENT_FEE};
use crate::error::NetError;

pub(in crate::net) fn with_asset(selector: [u8; 4], asset: u64) -> Vec<u8> {
    let mut out = selector.to_vec();
    out.extend_from_slice(&[0u8; 24]);
    out.extend_from_slice(&asset.to_be_bytes());
    out
}

pub(in crate::net) fn settlement_check(asset: u64, public_amount: u64, fee: u64) -> Vec<u8> {
    let mut out = with_asset(SETTLEMENT_FEE, asset);
    for v in [public_amount, fee, 1] {
        out.extend_from_slice(&[0u8; 24]);
        out.extend_from_slice(&v.to_be_bytes());
    }
    out
}

fn word(reply: &[u8], i: usize) -> Result<u64, NetError> {
    let at = i.checked_mul(32).ok_or(NetError::ReplyShape)?;
    let w = reply
        .get(at..at.checked_add(32).ok_or(NetError::ReplyShape)?)
        .ok_or(NetError::ReplyShape)?;
    let (high, low) = w.split_at(24);
    if high.iter().any(|b| *b != 0) {
        return Err(NetError::ReplyShape);
    }
    let mut be = [0u8; 8];
    be.copy_from_slice(low);
    Ok(u64::from_be_bytes(be))
}

/// `(uint64 protocolFee, uint64[4] ladder, bool set)`, refused when the schedule is not set.
pub(crate) fn schedule(reply: &[u8]) -> Result<Schedule, NetError> {
    if word(reply, 5)? != 1 {
        return Err(NetError::ReplyShape);
    }
    let ladder = [word(reply, 1)?, word(reply, 2)?, word(reply, 3)?, word(reply, 4)?];
    Ok(Schedule { protocol: word(reply, 0)?, ladder })
}

/// The withdrawal percentage of `bps()`, `(uint16 deposit, uint16 withdraw)`.
pub(in crate::net) fn bps(reply: &[u8]) -> Result<u16, NetError> {
    u16::try_from(word(reply, 1)?).map_err(|_| NetError::ReplyShape)
}
