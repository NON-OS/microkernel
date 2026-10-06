//! One typed amount shielded from the public account as several standard deposits under one review.
//! The pool is read and checked as for one deposit, each deposit gets its own note, and the notes
//! travel with the held review, each stored before its deposit leaves.

use super::super::{PendingBatch, Wallet};
use super::view::{plan, refused, ShieldSplit};
use crate::error::{NetError, WalletError};
use crate::evm::shield_batch::{review_batch, BatchChecked, Deposit};
use crate::evm::{self, Network};
use crate::ffi::amount_read::parse_base_units;
use crate::ffi::evm::parse_evm_address;
use crate::net::asset::Coin;
use crate::net::pool::{ACTIVE, RPCS};
use crate::wallet::deposit_split::pieces;
use crate::wallet::{check_deposit, prepare_deposit, read_pool_state_checked, DepositRequest};
use std::time::Instant;

impl Wallet {
    /// Work out `amount` of `coin` as the fewest standard deposits from the active account.
    pub fn review_shield_split(
        &self,
        coin: Coin,
        amount: String,
    ) -> Result<ShieldSplit, WalletError> {
        let asset = ACTIVE.asset(coin).ok_or(WalletError::Amount)?;
        let amount = parse_base_units(&amount)?;
        let units =
            amount.checked_div(asset.scale).filter(|_| amount.checked_rem(asset.scale) == Some(0));
        let sizes = units.and_then(|u| pieces(u, &asset)).ok_or(WalletError::NotStandard)?;
        let (from, to) = self.with(|s| (s.evm().address(), s.address()))?;
        let tor = self.tor()?;
        let state = read_pool_state_checked(&tor, &RPCS, &ACTIVE, &from, asset.id)?;
        if state.scale != asset.scale || state.beta_mode {
            return Ok(refused(
                "This pool takes these deposits one by one. Deposit each size alone.",
            ));
        }
        let mut requests: Vec<DepositRequest> = Vec::with_capacity(sizes.len());
        for size in sizes {
            let base = u128::from(size).checked_mul(asset.scale).ok_or(WalletError::Amount)?;
            if let Err(refusal) = check_deposit(&state, base) {
                return Ok(refused(super::super::ticket::sentence(refusal)));
            }
            requests.push(prepare_deposit(&to, base, asset.id, state.fee_bps, state.scale)?);
        }
        let deposits: Vec<Deposit> =
            requests.iter().map(|r| Deposit { value: r.value_wei, data: &r.calldata }).collect();
        let pool = parse_evm_address(ACTIVE.address)?;
        let token = asset.token.map(parse_evm_address).transpose()?;
        let sends = evm::nonce::recorded(&self.paths().account, Network::Sepolia, &from);
        let batch = match review_batch(&tor, &from, (&pool, token, amount), &deposits, &sends)? {
            BatchChecked::Ready(batch) => batch,
            BatchChecked::Approve => return self.approve_split(coin, amount, &requests),
            BatchChecked::Refused(why) => return Ok(refused(why)),
        };
        let id = self.next_review_id();
        let view = plan(id, &requests, asset.scale, batch.max_network_fee);
        let steps = batch.txs.into_iter().zip(requests.into_iter().map(|r| r.note)).collect();
        let held = PendingBatch { id, from, steps, made: Instant::now() };
        *self.batch.lock().map_err(|_| NetError::Transport)? = Some(held);
        Ok(view)
    }
}
