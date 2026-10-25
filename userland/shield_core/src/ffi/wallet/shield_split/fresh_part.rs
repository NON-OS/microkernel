//! The standard part of NOX or ETH at a fresh address (v1.4.56, v1.4.60): the sizes the
//! policy's rules() allow, or the asset's own when the pool has no policy,
//! and their sum in wei; the review says what is left at the address.

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
    /// The token for NOX; None for ETH, which goes as each deposit's value.
    pub token: Option<[u8; 20]>,
    pub sizes: Vec<u64>,
    pub sum: u128,
}

/// The part of `amount` wei of `coin` that goes as standard deposits;
/// None below the smallest.
pub(super) fn part(tor: &Tor, coin: Coin, amount: u128) -> Result<Option<Part>, WalletError> {
    let asset = ACTIVE.asset(coin).ok_or(WalletError::Amount)?;
    let (pool, token) = part_ends(coin)?;
    let units = amount.checked_div(asset.scale).ok_or(WalletError::Amount)?;
    let sized = match rule_now(tor, &ACTIVE, asset)? {
        Some(rule) => standard_part(units, |s| rule.standard(s)),
        None => standard_part(units, |s| asset.allows(s)),
    };
    let Some((sizes, _)) = sized else { return Ok(None) };
    let wide = sizes.iter().try_fold(0u128, |t, s| t.checked_add(u128::from(*s)));
    let sum = wide.and_then(|w| w.checked_mul(asset.scale)).ok_or(WalletError::Amount)?;
    Ok(Some(Part { asset, pool, token, sizes, sum }))
}

/// The pool and `coin`'s token, as the config names them: NOX has one,
/// ETH none, and no other coin moves from a fresh address.
pub(super) fn part_ends(coin: Coin) -> Result<([u8; 20], Option<[u8; 20]>), WalletError> {
    let asset = ACTIVE.asset(coin).ok_or(WalletError::Amount)?;
    let token = asset.token.map(parse_evm_address).transpose()?;
    match (coin, token) {
        (Coin::Nox, Some(_)) | (Coin::Eth, None) => Ok((parse_evm_address(ACTIVE.address)?, token)),
        _ => Err(WalletError::Amount),
    }
}
