//! SHIELD_FROM_FRESH's approval (v1.4.56): from the fresh address, of the
//! pool for exactly the deposits' sum, never more; confirmed alone, then
//! the deposits are reviewed again.

use super::super::ticket::approve_calldata;
use super::super::{Pending, Wallet};
use super::fresh_view::{said, FreshShield};
use super::view::{plan, refused, ShieldSplit};
use crate::error::{NetError, WalletError};
use crate::evm::shield::{review_shield, ShieldChecked, ShieldOrder, ETH_SHORT};
use crate::evm::{self, Network};
use crate::net::pool::ACTIVE;
use crate::net::tor::Tor;
use crate::wallet::fresh::shield_checked;
use crate::wallet::DepositRequest;
use std::time::Instant;

const MOVED: &str = "The allowance moved. Review it again.";

impl Wallet {
    pub(super) fn fresh_approve(
        &self,
        tor: &Tor,
        (from, pool, token): ([u8; 20], [u8; 20], [u8; 20]),
        (sum, remainder): (u128, u128),
        requests: &[DepositRequest],
    ) -> Result<FreshShield, WalletError> {
        let first = requests.first().ok_or(WalletError::Amount)?;
        let approve = approve_calldata(sum)?;
        let order = ShieldOrder {
            pool,
            token: Some(token),
            amount: sum,
            approve_data: &approve,
            deposit_data: &first.calldata,
            refused_as: "The pool would not take these deposits now. Nothing was sent.",
        };
        let sends = evm::nonce::recorded(&self.paths().account, Network::Sepolia, &from);
        let review = match review_shield(tor, &from, &order, &sends)? {
            ShieldChecked::Ready(review) if review.approval => review,
            ShieldChecked::Ready(_) => return Ok(said(MOVED, remainder)),
            ShieldChecked::ShortOfEth(lacks) => {
                return Ok(FreshShield { split: refused(ETH_SHORT), remainder, short_eth: lacks });
            }
            ShieldChecked::Refused(why) => return Ok(said(why, remainder)),
        };
        if let Err(why) =
            shield_checked(core::slice::from_ref(&review.tx), (pool, token), Some(&approve))
        {
            return Ok(said(why.why(), remainder));
        }
        let id = self.next_review_id();
        let scale = ACTIVE.asset(crate::net::asset::Coin::Nox).map_or(1, |a| a.scale);
        let split =
            ShieldSplit { approval: true, ..plan(id, requests, scale, review.max_network_fee) };
        let held = Pending {
            id,
            network: Network::Sepolia,
            from,
            tx: review.tx,
            made: Instant::now(),
            note: None,
        };
        *self.pending.lock().map_err(|_| NetError::Transport)? = Some(held);
        Ok(FreshShield { split, remainder, short_eth: 0 })
    }
}
