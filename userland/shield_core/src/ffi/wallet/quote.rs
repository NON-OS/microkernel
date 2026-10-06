//! The fee a spend pays, quoted before any proving from the policy of the pool and the latest base
//! fee, its network part and its protocol part apart. The quote is held, and a spend whose fee has
//! moved since is refused, so the fee proved is always a fee the owner was shown.

use super::quote_view::{refused, shown, SpendQuote};
use super::Wallet;
use crate::error::{NetError, WalletError};
use crate::ffi::amount_read::parse_amount;
use crate::net::asset::Coin;
use crate::net::fee_quote::quote;
use crate::net::pool::ACTIVE;

/// The order a quote was made for, coin, note units and whether it withdraws, and its fee.
pub(super) struct Quoted {
    order: (Coin, u64, bool),
    fee: u64,
}

impl Wallet {
    /// The fee a private transfer, or a withdrawal when `withdraw`, of `amount` of `coin` pays now.
    pub fn quote_spend(
        &self,
        coin: Coin,
        amount: String,
        withdraw: bool,
    ) -> Result<SpendQuote, WalletError> {
        self.with(|_| ())?;
        let asset = ACTIVE.asset(coin).ok_or(WalletError::Amount)?;
        let policy = ACTIVE.policy.ok_or(WalletError::Unavailable)?;
        let units = parse_amount(coin, amount)?;
        if !asset.allows(units) {
            return Err(WalletError::NotStandard);
        }
        let found = quote(self.tor()?.as_ref(), policy, asset, if withdraw { units } else { 0 })?;
        let fee = found.and_then(|q| q.total());
        let held = fee.map(|fee| Quoted { order: (coin, units, withdraw), fee });
        *self.quoted.lock().map_err(|_| NetError::Transport)? = held;
        Ok(found.map_or_else(refused, |q| shown(&q, asset)))
    }
}

impl Wallet {
    /// Refuse a spend of the quoted order whose fee is not the fee quoted. The quote is used up.
    pub(super) fn keep_to_quote(
        &self,
        order: (Coin, u64, bool),
        fee: u64,
    ) -> Result<(), WalletError> {
        let held = self.quoted.lock().map_err(|_| NetError::Transport)?.take();
        match held {
            Some(q) if q.order == order && q.fee != fee => Err(WalletError::FeeChanged),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
#[path = "quote_test.rs"]
mod quote_test;
