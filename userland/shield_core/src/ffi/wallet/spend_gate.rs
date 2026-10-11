//! What a spend must pass on the pool now, asked just before proving: a withdraw waits while the
//! policy is paused, and every amount must be a standard size by the pool's own isStandard (M3),
//! since a new range applies at activateRange with no grace.

use crate::error::WalletError;
use crate::net::asset::Asset;
use crate::net::policy_paused::policy_paused;
use crate::net::policy_standard::standard_now;
use crate::net::pool::Pool;
use crate::net::tor::Tor;

pub(super) fn may_spend(
    tor: &Tor,
    pool: &Pool,
    asset: Asset,
    units: u64,
    withdraw: bool,
) -> Result<(), WalletError> {
    if withdraw && policy_paused(tor, pool)? {
        return Err(WalletError::WithdrawalsPaused);
    }
    if !standard_now(tor, pool, asset, units)? {
        return Err(WalletError::NotStandard);
    }
    Ok(())
}
