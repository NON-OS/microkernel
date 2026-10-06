//! The earliest time a spend settles: the pool's ten-minute grid point just passed, so a proof can
//! settle as soon as it lands. Every spend proved in one slot carries the same time, which says
//! nothing about the wallet. A lander waits for the chain's clock when the phone's runs ahead.

use crate::error::{CustodyError, WalletError};
use std::time::{SystemTime, UNIX_EPOCH};

/// The pool's grid for the not-before time, in seconds.
pub const GRID: u64 = 600;

/// The not-before time for a spend proved now.
pub fn not_before() -> Result<u64, WalletError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| CustodyError::Entropy)?;
    Ok(on_grid(now.as_secs()))
}

/// `at` rounded down to the grid, and never zero.
pub fn on_grid(at: u64) -> u64 {
    at.checked_div(GRID).unwrap_or(0).max(1).saturating_mul(GRID)
}

#[cfg(test)]
#[path = "not_before_test.rs"]
mod not_before_test;
