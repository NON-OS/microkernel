//! Reading the pool state a deposit depends on, over the wallet's own Tor.
//!
//! Ten view calls, twelve on a launch pool, each decoded strictly: a reply that is not one ABI
//! word is refused, never read as zero, since a zero cap and a garbled reply mean different things.

use super::deposit_gate::PoolState;
use crate::error::NetError;
use crate::net::pool::{Pool, Shape};
use crate::net::rpc::{call_over_tor, calls_over_tor};
use crate::net::tor::{Purpose, Tor};

/// Selectors, from the compiled pool's method table.
const DEPOSITS_PAUSED: [u8; 4] = [0x60, 0xda, 0x3e, 0x83];
const WOUND_DOWN: [u8; 4] = [0x1c, 0x39, 0x6d, 0xa3];
const BETA_MODE: [u8; 4] = [0x8a, 0xbe, 0x2f, 0x26];
const BETA_PAUSED: [u8; 4] = [0x8a, 0xe0, 0xb0, 0x58];
const BETA_DEPOSITOR: [u8; 4] = [0x51, 0xcc, 0x9f, 0x48];
const BETA_DEPOSITED: [u8; 4] = [0x58, 0xdb, 0x32, 0x5c];
const BETA_ADDR_CAP: [u8; 4] = [0x23, 0x38, 0x53, 0xbf];
const BETA_TOTAL_DEPOSITED: [u8; 4] = [0x83, 0xf9, 0x56, 0xe2];
const BETA_TOTAL_CAP: [u8; 4] = [0xf3, 0x0e, 0x5e, 0x0f];
const SHIELD_FEE_BPS: [u8; 4] = [0x54, 0x12, 0x52, 0xf9];
/// `bps()` of an amount policy, which holds the deposit fee once a pool has one.
const POLICY_BPS: [u8; 4] = [0x68, 0x23, 0x73, 0x29];
/// Launch shape only: `scale(uint64)` and `openDeposits()`.
const SCALE: [u8; 4] = [0x1e, 0x89, 0xff, 0x8f];
const OPEN_DEPOSITS: [u8; 4] = [0xc3, 0xf8, 0x1c, 0xc5];

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
    let (scale, open_deposits) = if launch {
        let [scale, open] =
            <[[u8; 32]; 2]>::try_from(words.split_off(base)).map_err(|_| NetError::ReplyShape)?;
        (amount(scale), flag(open)?)
    } else {
        (1, false)
    };
    let fee = match pool.policy {
        Some(policy) => {
            let reply = call_over_tor(tor, Purpose::Deposit, host, policy, &POLICY_BPS)?;
            <[u8; 32]>::try_from(reply.get(..32).ok_or(NetError::ReplyShape)?)
                .map_err(|_| NetError::ReplyShape)?
        }
        None => words.pop().ok_or(NetError::ReplyShape)?,
    };
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
    })
}

/// The pool state from every RPC in `hosts` that answers. Two answers must agree, since a lone
/// RPC could report a wound-down pool as open and trap a deposit. Disagreement refuses the read.
pub fn read_pool_state_checked(
    tor: &Tor,
    hosts: &[&str],
    pool: &Pool,
    depositor: &[u8; 20],
    asset: u64,
) -> Result<PoolState, NetError> {
    let mut agreed: Option<PoolState> = None;
    let mut last = NetError::Transport;
    for host in hosts {
        match read_pool_state(tor, host, pool, depositor, asset) {
            Ok(state) => match agreed {
                None => agreed = Some(state),
                Some(prior) if prior == state => {}
                Some(_) => return Err(NetError::ReplyShape),
            },
            Err(e) => last = e,
        }
    }
    agreed.ok_or(last)
}

fn call(selector: &[u8; 4], args: &[[u8; 32]]) -> Vec<u8> {
    let mut out = selector.to_vec();
    for arg in args {
        out.extend_from_slice(arg);
    }
    out
}

fn asset_word(asset: u64) -> [u8; 32] {
    let mut w = [0u8; 32];
    let (_, low) = w.split_at_mut(24);
    low.copy_from_slice(&asset.to_be_bytes());
    w
}

fn address_word(addr: &[u8; 20]) -> [u8; 32] {
    let mut w = [0u8; 32];
    let (_, low) = w.split_at_mut(12);
    low.copy_from_slice(addr);
    w
}

/// An ABI bool: zero or one, and nothing else.
fn flag(word: [u8; 32]) -> Result<bool, NetError> {
    let (high, last) = word.split_at(31);
    match (high.iter().all(|b| *b == 0), last) {
        (true, [0]) => Ok(false),
        (true, [1]) => Ok(true),
        _ => Err(NetError::ReplyShape),
    }
}

/// An ABI uint256 as a u128, saturating. A cap beyond u128 is no cap at all.
fn amount(word: [u8; 32]) -> u128 {
    let (high, low) = word.split_at(16);
    if high.iter().any(|b| *b != 0) {
        return u128::MAX;
    }
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(low);
    u128::from_be_bytes(bytes)
}
