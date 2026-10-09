//! Reading an amount a user typed. Anything not a decimal number in range is
//! refused, so no shell decides what a malformed amount means. A comma reads as
//! a decimal point, because half the world types it that way.

use super::amount::{scale_of, DECIMALS};
use crate::error::WalletError;
use crate::net::asset::Coin;

/// Read an amount of `coin` a user typed as the active pool's note units,
/// refusing anything out of range or not a whole number of units.
pub fn parse_amount(coin: Coin, text: String) -> Result<u64, WalletError> {
    let scale = scale_of(coin);
    if scale == 1 {
        let units = parse_base_amount(&text)?;
        return (units <= crate::notes::MAX_VALUE).then_some(units).ok_or(WalletError::Amount);
    }
    let base = parse_base_units(&text)?;
    if base.checked_rem(scale) != Some(0) {
        return Err(WalletError::Amount);
    }
    let units = u64::try_from(base.checked_div(scale).ok_or(WalletError::Amount)?)
        .map_err(|_| WalletError::Amount)?;
    (units <= crate::notes::MAX_VALUE).then_some(units).ok_or(WalletError::Amount)
}

/// Base units on a one-for-one pool, through the verified kernel.
fn parse_base_amount(text: &str) -> Result<u64, WalletError> {
    let cleaned = text.trim().replace(',', ".");
    let (whole, fraction) = match cleaned.split_once('.') {
        Some((w, f)) => (w, f),
        None => (cleaned.as_str(), ""),
    };
    if whole.is_empty() && fraction.is_empty() {
        return Err(WalletError::Amount);
    }
    if fraction.len() > DECIMALS as usize {
        return Err(WalletError::Amount);
    }
    let digits = |s: &str| s.chars().all(|c| c.is_ascii_digit());
    if !digits(whole) || !digits(fraction) {
        return Err(WalletError::Amount);
    }
    let units: u64 =
        if whole.is_empty() { 0 } else { whole.parse().map_err(|_| WalletError::Amount)? };
    let padded = format!("{fraction:0<18}");
    let sub: u64 =
        if fraction.is_empty() { 0 } else { padded.parse().map_err(|_| WalletError::Amount)? };
    // The verified kernel refuses to wrap, as a wrapped product is a smaller balance.
    nox_verified::amount::combine(units, sub).ok_or(WalletError::Amount)
}

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

/// The coin's name, for the label a screen puts next to a number.
pub fn amount_unit(coin: Coin) -> String {
    coin.symbol().into()
}
