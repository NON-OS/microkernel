//! SHIELD_FROM_FRESH's review (v1.4.56): NOX found at a fresh address this
//! wallet handed out, moved into the Shield from that address as standard
//! deposits. What is below the smallest standard size stays there; an
//! approval of exactly the deposits' sum comes first; the gas is the
//! address's own ETH, and when it lacks some the review says how much.

use super::super::{PendingBatch, Wallet};
use super::fresh_part::part;
use super::fresh_view::{said, FreshShield};
use super::view::{plan, refused};
use crate::error::{NetError, WalletError};
use crate::evm::shield_batch::{review_batch, BatchChecked, Deposit, BATCH_ETH_SHORT};
use crate::evm::{self, Network};
use crate::wallet::fresh::{handed_index, shield_checked, ShieldRefusal};
use std::time::Instant;

impl Wallet {
    /// Review moving `amount` NOX wei at `address` into the Shield.
    pub fn review_shield_from_fresh(
        &self,
        address: [u8; 20],
        amount: u128,
    ) -> Result<FreshShield, WalletError> {
        let handed = handed_index(&self.paths().account, &address);
        if handed.is_none() || self.with(|s| s.imported)? {
            return Ok(said(ShieldRefusal::UnknownAddress.why(), 0));
        }
        let tor = self.tor()?;
        let Some(part) = part(&tor, amount)? else {
            return Ok(said(ShieldRefusal::BelowStandard.why(), amount));
        };
        let (pool, token, sum, remainder) = (part.pool, part.token, part.sum, part.remainder);
        let requests = match self.fresh_requests(&tor, &address, part.asset, &part.sizes)? {
            Ok(requests) => requests,
            Err(why) => return Ok(said(why, remainder)),
        };
        let deposits: Vec<Deposit> =
            requests.iter().map(|r| Deposit { value: r.value_wei, data: &r.calldata }).collect();
        let sends = evm::nonce::recorded(&self.paths().account, Network::Sepolia, &address);
        let batch =
            match review_batch(&tor, &address, (&pool, Some(token), sum), &deposits, &sends)? {
                BatchChecked::Ready(batch) => batch,
                BatchChecked::Approve => {
                    return self.fresh_approve(
                        &tor,
                        (address, pool, token),
                        (sum, remainder),
                        &requests,
                    );
                }
                BatchChecked::ShortOfEth(lacks) => {
                    return Ok(FreshShield {
                        split: refused(BATCH_ETH_SHORT),
                        remainder,
                        short_eth: lacks,
                    });
                }
                BatchChecked::Refused(why) => return Ok(said(why, remainder)),
            };
        if let Err(why) = shield_checked(&batch.txs, (pool, token), None) {
            return Ok(said(why.why(), remainder));
        }
        let id = self.next_review_id();
        let split = plan(id, &requests, part.asset.scale, batch.max_network_fee);
        let steps = batch.txs.into_iter().zip(requests.into_iter().map(|r| r.note)).collect();
        let held = PendingBatch { id, from: address, steps, made: Instant::now() };
        *self.batch.lock().map_err(|_| NetError::Transport)? = Some(held);
        Ok(FreshShield { split, remainder, short_eth: 0 })
    }
}
