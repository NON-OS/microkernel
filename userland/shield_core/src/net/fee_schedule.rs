//! The fee of a spend on a pool with a fee schedule: the protocol part, flat for a private
//! transfer and a percentage of the amount for a withdrawal, plus one rung, and one only, of the gas
//! ladder. Every figure is read from the pool's policy over Tor, and the policy's own check is
//! asked last, so a fee the pool would refuse is refused before any proving time is spent.

use super::asset::Asset;
use super::pool::Pool;
use super::tor::Tor;
use crate::error::NetError;

/// `scheduleOf(uint64)`, `bps()` and `settlementFee(uint64,uint256,uint256,bool)`.
pub(super) const SCHEDULE_OF: [u8; 4] = [0x01, 0xe7, 0xd8, 0xd8];
pub(super) const BPS: [u8; 4] = [0x68, 0x23, 0x73, 0x29];
const SETTLEMENT_FEE: [u8; 4] = [0x73, 0xd8, 0xe9, 0x32];
/// Every spend pays the lowest rung, at which the lander settles, as the pool operators set it.
pub const RUNG: usize = 0;

/// A schedule as the policy holds it: the flat protocol part and the four rungs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Schedule {
    pub protocol: u64,
    pub ladder: [u64; 4],
}

/// The fee of a spend of `asset` with `public_amount` out of the pool, zero for a private
/// transfer, or none when the pool's policy refuses it. A pool with no policy keeps its flat fee.
pub fn spend_fee(
    tor: &Tor,
    pool: &Pool,
    asset: Asset,
    public_amount: u64,
) -> Result<Option<u64>, NetError> {
    let Some(policy) = pool.policy else { return Ok(Some(asset.relay_fee)) };
    Ok(super::fee_quote::quote(tor, policy, asset, public_amount)?.and_then(|q| q.total()))
}

/// The protocol part and rung `at` of `sc`, or none off the ladder or when the sum would wrap.
pub fn split(sc: Schedule, at: usize, withdraw_bps: u16, public_amount: u64) -> Option<(u64, u64)> {
    let protocol = if public_amount == 0 {
        sc.protocol
    } else {
        let part =
            u128::from(public_amount).checked_mul(u128::from(withdraw_bps))?.checked_div(10_000)?;
        u64::try_from(part).ok()?
    };
    let network = *sc.ladder.get(at)?;
    protocol.checked_add(network).map(|_| (protocol, network))
}

/// The protocol part plus rung `at`, in note units.
pub fn fee(sc: Schedule, at: usize, withdraw_bps: u16, public_amount: u64) -> Option<u64> {
    split(sc, at, withdraw_bps, public_amount).and_then(|(p, n)| p.checked_add(n))
}

pub(super) fn with_asset(selector: [u8; 4], asset: u64) -> Vec<u8> {
    let mut out = selector.to_vec();
    out.extend_from_slice(&[0u8; 24]);
    out.extend_from_slice(&asset.to_be_bytes());
    out
}

pub(super) fn settlement_check(asset: u64, public_amount: u64, fee: u64) -> Vec<u8> {
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
pub(super) fn bps(reply: &[u8]) -> Result<u16, NetError> {
    u16::try_from(word(reply, 1)?).map_err(|_| NetError::ReplyShape)
}

#[cfg(test)]
#[path = "fee_schedule_test.rs"]
mod fee_schedule_test;
