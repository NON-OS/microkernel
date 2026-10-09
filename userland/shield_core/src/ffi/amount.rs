//! Amounts, in and out, so the shells do no arithmetic on money. A note holds a
//! u64 of note units, each `scale` base units, and a larger amount is several notes.

/// Decimal places of NOX and ETH. The pool counts note units, a screen whole tokens.
pub const DECIMALS: u32 = 18;

use crate::net::asset::Coin;
use crate::net::pool::ACTIVE;

/// The active pool's scale for `coin`. A missing entry is a build error a test catches.
pub(crate) fn scale_of(coin: Coin) -> u128 {
    ACTIVE.asset(coin).map_or(1, |a| a.scale)
}

/// Format `coin` note units for a screen, with the verified kernel's split that
/// `lean/AmountProofs.lean` proves, or checked arithmetic for a scaled asset.
pub fn format_amount(coin: Coin, units: u64) -> String {
    let scale = scale_of(coin);
    if scale != 1 {
        return u128::from(units)
            .checked_mul(scale)
            .map_or_else(|| "?".into(), super::chain_types::format_wide);
    }
    let (whole, fraction) = nox_verified::amount::split(units);
    if fraction == 0 {
        return whole.to_string();
    }
    let mut text = format!("{whole}.{fraction:018}");
    while text.ends_with('0') {
        text.pop();
    }
    text
}

/// The most one note of `coin` holds, as a screen shows it: `MAX_VALUE`, p - 2 units.
pub fn max_note_amount(coin: Coin) -> String {
    format_amount(coin, crate::notes::MAX_VALUE)
}
