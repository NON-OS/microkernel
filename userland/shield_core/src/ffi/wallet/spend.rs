//! Send and withdraw on the active pool. The fee is read from the pool's policy and the history
//! over Tor without the session lock, the spend is proved under it, and the hand-off comes back
//! paying whoever lands it, from the pool's ten-minute grid point just passed.

#[path = "spend_run.rs"]
mod run;

use super::Wallet;
use crate::error::WalletError;
use crate::ffi::chain_types::SpendTicket;
use crate::ffi::evm::parse_evm_address;
use crate::ffi::CancelToken;
use crate::keys::parse_receiving_address;
use crate::net::asset::Coin;
use crate::wallet::spend::Destination;
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
