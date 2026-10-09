//! A deposit from the user's own EVM wallet. The live pool is read from every RPC
//! that answers and the amount runs the contract's own checks in order. A pool that
//! would refuse or keep the money gets a refusal. Otherwise the note is stored before
//! any transaction leaves, since a deposit whose blinding is lost cannot be spent.

use super::ticket::{refused, ticket};
use super::Wallet;
use crate::error::WalletError;
use crate::ffi::amount_read::parse_base_units;
use crate::ffi::chain_types::{format_wide, DepositTicket, Recoverable};
use crate::ffi::evm::parse_evm_address;
use crate::net::asset::Coin;
use crate::net::pool::{ACTIVE, RPCS};
use crate::store::Row;
use crate::wallet::{check_deposit, prepare_deposit, read_pool_state_checked, read_recoverable};

impl Wallet {
    /// Check a deposit of `amount` of `coin` from `depositor` against the live pool,
    /// and if it lands, keep its note and return the transactions to sign.
    pub fn prepare_deposit(
        &self,
        coin: Coin,
        amount: String,
        depositor: String,
    ) -> Result<DepositTicket, WalletError> {
        let asset = ACTIVE.asset(coin).ok_or(WalletError::Amount)?;
        let amount = parse_base_units(&amount)?;
        // A deposit is public, so it is a standard size inside the pool's range, whole note units.
        let units = u64::try_from(amount.checked_div(asset.scale).unwrap_or(0)).unwrap_or(0);
        if amount.checked_rem(asset.scale) != Some(0) || !asset.allows(units) {
            return Err(WalletError::NotStandard);
        }
        let from = parse_evm_address(&depositor)?;
        let to = self.with(|s| s.address())?;
        let tor = self.tor()?;
        let state = read_pool_state_checked(&tor, &RPCS, &ACTIVE, &from, asset.id)?;
        // A scale unlike this build's would skew every balance, so the deposit waits.
        if state.scale != asset.scale {
            return Ok(refused(
                amount,
                "This pool counts this coin differently from this build. Update the app first.",
            ));
        }
        if let Err(refusal) = check_deposit(&state, amount) {
            return Ok(refused(amount, super::ticket::sentence(refusal)));
        }
        let request = prepare_deposit(&to, amount, asset.id, state.fee_bps, state.scale)?;
        self.with_mut(|s| Ok(s.record(Row::Deposit(request.note.clone()))?))?;
        ticket(&request, &asset)
    }

    /// What `owner` could claim from the pool and take back from beta, in `coin`.
    pub fn recoverable(&self, coin: Coin, owner: String) -> Result<Recoverable, WalletError> {
        let asset = ACTIVE.asset(coin).ok_or(WalletError::Amount)?;
        let owner = parse_evm_address(&owner)?;
        let tor = self.tor()?;
        let (claimable, refundable) =
            read_recoverable(&tor, &RPCS, ACTIVE.address, &owner, asset.id)?;
        Ok(Recoverable { claimable: format_wide(claimable), refundable: format_wide(refundable) })
    }
}

#[cfg(test)]
#[path = "deposit_test.rs"]
mod deposit_test;
