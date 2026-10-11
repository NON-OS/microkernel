//! The standard part of NOX at a fresh address (v1.4.56): the sizes the
//! policy's rules() allow, or the asset's own when the pool has no policy,
//! their sum in wei, and what is left at the address.

use crate::error::WalletError;
use crate::ffi::evm::parse_evm_address;
use crate::net::asset::{Asset, Coin};
use crate::net::policy_standard::rule_now;
use crate::net::pool::ACTIVE;
use crate::net::tor::Tor;
use crate::wallet::fresh::standard_part;

pub(super) struct Part {
    pub asset: Asset,
    pub pool: [u8; 20],
    pub token: [u8; 20],
    pub sizes: Vec<u64>,
    pub sum: u128,
    pub remainder: u128,
}

/// The part of `amount` wei that goes as standard deposits; None below the smallest.
pub(super) fn part(tor: &Tor, amount: u128) -> Result<Option<Part>, WalletError> {
    let asset = ACTIVE.asset(Coin::Nox).ok_or(WalletError::Amount)?;
    let (pool, token) = part_ends()?;
    let units = amount.checked_div(asset.scale).ok_or(WalletError::Amount)?;
    let sized = match rule_now(tor, &ACTIVE, asset)? {
        Some(rule) => standard_part(units, |s| rule.standard(s)),
        None => standard_part(units, |s| asset.allows(s)),
    };
    let Some((sizes, _)) = sized else { return Ok(None) };
    let wide = sizes.iter().try_fold(0u128, |t, s| t.checked_add(u128::from(*s)));
    let sum = wide.and_then(|w| w.checked_mul(asset.scale)).ok_or(WalletError::Amount)?;
    let remainder = amount.checked_sub(sum).ok_or(WalletError::Amount)?;
    Ok(Some(Part { asset, pool, token, sizes, sum, remainder }))
}

/// The pool and the NOX token, as the config names them.
pub(super) fn part_ends() -> Result<([u8; 20], [u8; 20]), WalletError> {
    let asset = ACTIVE.asset(Coin::Nox).ok_or(WalletError::Amount)?;
    let token = asset.token.map(parse_evm_address).transpose()?.ok_or(WalletError::Amount)?;
    Ok((parse_evm_address(ACTIVE.address)?, token))
}
