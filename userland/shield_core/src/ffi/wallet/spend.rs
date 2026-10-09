//! Send and withdraw on the active pool. The fee is read from the pool's policy and the history
//! over Tor without the session lock, the spend is proved under it, and the hand-off comes back
//! paying whoever lands it, from the pool's ten-minute grid point just passed.

use super::Wallet;
use crate::error::WalletError;
use crate::ffi::amount_read::parse_amount;
use crate::ffi::chain_types::SpendTicket;
use crate::ffi::evm::parse_evm_address;
use crate::ffi::CancelToken;
use crate::keys::parse_receiving_address;
use crate::net::asset::Coin;
use crate::net::pool::ACTIVE;
use crate::net::rpc::progress::Stop;
use crate::wallet::spend::{spend, Destination, Order};
use std::sync::Arc;

impl Wallet {
    /// A private transfer of `coin` to another wallet's receiving address.
    pub fn send_private(
        &self,
        coin: Coin,
        to: String,
        amount: String,
        early: bool,
        cancel: Arc<CancelToken>,
    ) -> Result<SpendTicket, WalletError> {
        let (spend_pk, sealed_to) = parse_receiving_address(&to)?;
        let to = Destination::Wallet { spend_pk, sealed_to: Box::new(sealed_to) };
        self.run_spend(coin, to, (&amount, early), &cancel)
    }

    pub fn withdraw(
        &self,
        coin: Coin,
        to: String,
        amount: String,
        early: bool,
        cancel: Arc<CancelToken>,
    ) -> Result<SpendTicket, WalletError> {
        let to = Destination::Withdraw { recipient: parse_evm_address(&to)? };
        self.run_spend(coin, to, (&amount, early), &cancel)
    }
}

impl Wallet {
    fn run_spend(
        &self,
        coin: Coin,
        to: Destination,
        (amount, early): (&str, bool),
        cancel: &CancelToken,
    ) -> Result<SpendTicket, WalletError> {
        let asset = ACTIVE.asset(coin).ok_or(WalletError::Amount)?;
        let amount = parse_amount(coin, amount.into())?;
        // The pool's own fee, checked by its policy before any proving, paid to whoever lands it.
        let tor = self.tor()?;
        let public = if matches!(to, Destination::Withdraw { .. }) { amount } else { 0 };
        let fee = crate::net::fee_schedule::spend_fee(tor.as_ref(), &ACTIVE, asset, public)?
            .ok_or(WalletError::FeeTooHigh)?;
        self.keep_to_quote((coin, amount, public != 0), fee)?;
        let not_before = crate::wallet::spend::not_before::not_before()?;
        let fee_to = crate::net::pool::SUBMITTER;
        let order = Order { asset, to, amount, fee, fee_to, not_before, early };
        let history = self.pool_history(tor.as_ref(), Stop::never())?;
        let dir =
            self.paths().measurement.parent().map(std::path::Path::to_path_buf).unwrap_or_default();
        let outcome = self.with_mut(|s| spend(s, &history, &order, &dir, cancel.token()))?;
        Ok(SpendTicket {
            handoff_files: outcome
                .handoff
                .iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect(),
            weakened: outcome.weakened.iter().map(|w| (*w).to_string()).collect(),
        })
    }
}
