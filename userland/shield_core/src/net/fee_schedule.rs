//! The fee of a spend on a pool with a fee schedule: the protocol part, flat for a private
//! transfer and a percentage of the amount for a withdrawal, plus one rung, and one only, of the gas
//! ladder. Every figure is read from the pool's policy over Tor, and the policy's own check is
//! asked last, so a fee the pool would refuse is refused before any proving time is spent.

#[path = "fee_schedule_abi.rs"]
mod abi;

pub(crate) use abi::schedule;
pub(super) use abi::{bps, settlement_check, with_asset};

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

#[cfg(test)]
#[path = "fee_schedule_test.rs"]
mod fee_schedule_test;
