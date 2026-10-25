//! One try of a SHIELD_FROM_FRESH review (v1.4.56, v1.4.60): the deposits
//! of a part checked against the pool as read for the fresh address, the
//! batch built from it, checked again, and held for confirm; or the
//! approval NOX needs first, or the ETH the gas lacks.

use super::super::{PendingBatch, Wallet};
use super::fresh_part::Part;
use super::fresh_view::{said, FreshShield};
use super::view::plan;
use crate::error::{NetError, WalletError};
use crate::evm::shield_batch::{review_batch, BatchChecked, Deposit};
use crate::evm::{self, Network};
use crate::net::tor::Tor;
use crate::wallet::fresh::shield_checked;
use std::time::Instant;

const NO_APPROVAL: &str = "ETH needs no approval; the pool did not take these deposits.";

pub(super) enum Batched {
    Done(FreshShield),
    /// The ETH the address lacks for these deposits and their gas.
    Short(u128),
}

impl Wallet {
    pub(super) fn fresh_try(
        &self,
        tor: &Tor,
        address: [u8; 20],
        part: &Part,
        remainder: u128,
    ) -> Result<Batched, WalletError> {
        let requests = match self.fresh_requests(tor, &address, part.asset, &part.sizes)? {
            Ok(requests) => requests,
            Err(why) => return Ok(Batched::Done(said(why, remainder))),
        };
        let deposits: Vec<Deposit> =
            requests.iter().map(|r| Deposit { value: r.value_wei, data: &r.calldata }).collect();
        let value = deposits.iter().try_fold(0u128, |t, d| t.checked_add(d.value));
        let value = value.ok_or(WalletError::Amount)?;
        let sends = evm::nonce::recorded(&self.paths().account, Network::Sepolia, &address);
        let ends = (&part.pool, part.token, part.sum);
        let batch = match (review_batch(tor, &address, ends, &deposits, &sends)?, part.token) {
            (BatchChecked::Ready(batch), _) => batch,
            (BatchChecked::Approve, Some(token)) => {
                let ends = (address, part.pool, token);
                let approved = self.fresh_approve(tor, ends, (part.sum, remainder), &requests);
                return approved.map(Batched::Done);
            }
            (BatchChecked::Approve, None) => {
                return Ok(Batched::Done(said(NO_APPROVAL, remainder)))
            }
            (BatchChecked::ShortOfEth(lacks), _) => return Ok(Batched::Short(lacks)),
            (BatchChecked::Refused(why), _) => return Ok(Batched::Done(said(why, remainder))),
        };
        if let Err(why) = shield_checked(&batch.txs, (part.pool, part.token), None, value) {
            return Ok(Batched::Done(said(why.why(), remainder)));
        }
        let id = self.next_review_id();
        let split = plan(id, &requests, part.asset.scale, batch.max_network_fee);
        let steps = batch.txs.into_iter().zip(requests.into_iter().map(|r| r.note)).collect();
        let held = PendingBatch { id, from: address, steps, made: Instant::now() };
        *self.batch.lock().map_err(|_| NetError::Transport)? = Some(held);
        Ok(Batched::Done(FreshShield { split, remainder, short_eth: 0 }))
    }
}
