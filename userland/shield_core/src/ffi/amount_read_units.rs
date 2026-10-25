//! An amount typed, in base units wide enough for 10^20, with checked arithmetic.

use super::super::amount::DECIMALS;
use crate::error::WalletError;

/// An amount typed for a scaled pool, in base units wide enough for 10^20, past a
/// u64, with the text rules of `parse_amount` and checked arithmetic.
pub(crate) fn parse_base_units(text: &str) -> Result<u128, WalletError> {
    parse_units(text, DECIMALS)
}

/// An amount typed, in base units of a coin with `decimals` places, wei for ETH.
pub(crate) fn parse_units(text: &str, decimals: u32) -> Result<u128, WalletError> {
    let cleaned = text.trim().replace(',', ".");
    let (whole, fraction) = cleaned.split_once('.').unwrap_or((cleaned.as_str(), ""));
    let digits = |s: &str| s.chars().all(|c| c.is_ascii_digit());
    if (whole.is_empty() && fraction.is_empty())
        || fraction.len() > decimals as usize
        || !digits(whole)
        || !digits(fraction)
    {
        return Err(WalletError::Amount);
    }
    let number = |s: &str| if s.is_empty() { Ok(0) } else { s.parse::<u128>() };
    let whole = number(whole).map_err(|_| WalletError::Amount)?;
    let width = decimals as usize;
    let fraction = number(&format!("{fraction:0<width$}")).map_err(|_| WalletError::Amount)?;
    whole
        .checked_mul(10u128.checked_pow(decimals).ok_or(WalletError::Amount)?)
        .and_then(|w| w.checked_add(fraction))
        .ok_or(WalletError::Amount)
}
