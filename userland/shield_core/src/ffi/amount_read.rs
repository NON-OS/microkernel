//! Reading an amount a user typed. Anything not a decimal number in range is
//! refused, so no shell decides what a malformed amount means. A comma reads as
//! a decimal point, because half the world types it that way.

#[path = "amount_read_units.rs"]
mod units;

pub(crate) use units::{parse_base_units, parse_units};

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

/// The coin's name, for the label a screen puts next to a number.
pub fn amount_unit(coin: Coin) -> String {
    coin.symbol().into()
}
