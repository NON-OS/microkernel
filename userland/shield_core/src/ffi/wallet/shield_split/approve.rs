//! The exact approval of the whole amount a split deposit needs first, when the pool may not pull
//! it all yet. It is one send, confirmed with `confirm_public_send`, and the split is then reviewed
//! again with fresh notes.

use super::super::ticket::approve_calldata;
use super::super::{Pending, Wallet};
use super::view::{plan, refused, ShieldSplit};
use crate::error::{NetError, WalletError};
use crate::evm::shield::{review_shield, ShieldChecked, ShieldOrder};
use crate::evm::{self, Network};
use crate::ffi::evm::parse_evm_address;
use crate::net::asset::Coin;
use crate::net::pool::ACTIVE;
use crate::wallet::DepositRequest;
use std::time::Instant;

impl Wallet {
    pub(super) fn approve_split(
        &self,
        coin: Coin,
        amount: u128,
        requests: &[DepositRequest],
    ) -> Result<ShieldSplit, WalletError> {
        let asset = ACTIVE.asset(coin).ok_or(WalletError::Amount)?;
        let first = requests.first().ok_or(WalletError::Amount)?;
        let from = self.with(|s| s.evm().address())?;
        let order = ShieldOrder {
            pool: parse_evm_address(ACTIVE.address)?,
            token: asset.token.map(parse_evm_address).transpose()?,
            amount,
            approve_data: &approve_calldata(amount)?,
            deposit_data: &first.calldata,
            refused_as: "The pool would not take these deposits now. Nothing was sent.",
        };
        let sends = evm::nonce::recorded(&self.paths().account, Network::Sepolia, &from);
        let review = match review_shield(self.tor()?.as_ref(), &from, &order, &sends)? {
            ShieldChecked::Ready(review) if review.approval => review,
            ShieldChecked::Ready(_) => return Ok(refused("The allowance moved. Review it again.")),
            ShieldChecked::Refused(why) => return Ok(refused(why)),
        };
        let id = self.next_review_id();
        let view = ShieldSplit {
            approval: true,
            ..plan(id, requests, asset.scale, review.max_network_fee)
        };
        let held = Pending {
            id,
            network: Network::Sepolia,
            from,
            tx: review.tx,
            made: Instant::now(),
            note: None,
        };
        *self.pending.lock().map_err(|_| NetError::Transport)? = Some(held);
        Ok(view)
    }
}
