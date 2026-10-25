//! One RPC's pool state for a deposit, read in one batched request.

use super::super::deposit_gate::PoolState;
use super::abi::{address_word, amount, asset_word, call, flag};
use super::selectors::{BETA_ADDR_CAP, BETA_DEPOSITED, BETA_DEPOSITOR, BETA_MODE, BETA_PAUSED};
use super::selectors::{BETA_TOTAL_CAP, BETA_TOTAL_DEPOSITED, DEPOSITS_PAUSED, OPEN_DEPOSITS};
use super::selectors::{SCALE, SHIELD_FEE_BPS, WOUND_DOWN};
use super::shape::{fee_and_pause, launch_terms};
use crate::error::NetError;
use crate::net::pool::{Pool, Shape};
use crate::net::rpc::calls_over_tor;
use crate::net::tor::{Purpose, Tor};

/// The state of `pool` for `depositor` (a 20-byte EVM address) and `asset`.
/// A format 5 pool has no scale and no open deposits, and is read as such.
pub fn read_pool_state(
    tor: &Tor,
    host: &str,
    pool: &Pool,
    depositor: &[u8; 20],
    asset: u64,
) -> Result<PoolState, NetError> {
    let a = asset_word(asset);
    let d = address_word(depositor);
    let mut calls = vec![
        call(&DEPOSITS_PAUSED, &[]),
        call(&WOUND_DOWN, &[]),
        call(&BETA_MODE, &[]),
        call(&BETA_PAUSED, &[]),
        call(&BETA_DEPOSITOR, &[d]),
        call(&BETA_DEPOSITED, &[a, d]),
        call(&BETA_ADDR_CAP, &[a]),
        call(&BETA_TOTAL_DEPOSITED, &[a]),
        call(&BETA_TOTAL_CAP, &[a]),
    ];
    if pool.policy.is_none() {
        calls.push(call(&SHIELD_FEE_BPS, &[]));
    }
    let base = calls.len();
    let launch = pool.shape == Shape::Launch;
    if launch {
        calls.push(call(&SCALE, &[a]));
        calls.push(call(&OPEN_DEPOSITS, &[]));
    }
    // One batched request: every read comes from one connection, one block.
    let mut words = calls_over_tor(tor, Purpose::Deposit, host, pool.address, &calls)?
        .into_iter()
        .map(|w| <[u8; 32]>::try_from(w.as_slice()).map_err(|_| NetError::ReplyShape))
        .collect::<Result<Vec<_>, _>>()?;
    if words.len() != calls.len() {
        return Err(NetError::ReplyShape);
    }
    let (scale, open_deposits) = launch_terms(&mut words, base, launch)?;
    let (fee, policy_paused) = fee_and_pause(tor, host, pool, &mut words)?;
    let [paused, wound, mode, bpaused, listed, used, cap, total_used, total_cap] =
        <[[u8; 32]; 9]>::try_from(words).map_err(|_| NetError::ReplyShape)?;
    Ok(PoolState {
        open_deposits,
        scale,
        deposits_paused: flag(paused)?,
        wound_down: flag(wound)?,
        beta_mode: flag(mode)?,
        beta_paused: flag(bpaused)?,
        is_depositor: flag(listed)?,
        addr_deposited: amount(used),
        addr_cap: amount(cap),
        total_deposited: amount(total_used),
        total_cap: amount(total_cap),
        fee_bps: u16::try_from(amount(fee)).map_err(|_| NetError::ReplyShape)?,
        policy_paused,
    })
}
