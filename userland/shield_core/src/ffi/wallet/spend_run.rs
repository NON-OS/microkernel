//! One spend run: the fee read and checked, the history read, the spend proved under the lock.

use super::super::Wallet;
use crate::error::WalletError;
use crate::ffi::amount_read::parse_amount;
use crate::ffi::chain_types::SpendTicket;
use crate::ffi::CancelToken;
use crate::net::asset::Coin;
use crate::net::pool::ACTIVE;
use crate::net::rpc::progress::Stop;
use crate::wallet::spend::{spend, Destination, Order};

impl Wallet {
    pub(super) fn run_spend(
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
        super::super::spend_gate::may_spend(tor.as_ref(), &ACTIVE, asset, amount, public != 0)?;
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
