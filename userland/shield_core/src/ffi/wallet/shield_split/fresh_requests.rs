//! The deposits of a SHIELD_FROM_FRESH review (v1.4.56): each standard size
//! checked against the pool as read for the fresh address, each with its
//! own note to this wallet's shielded address.

use super::super::Wallet;
use crate::error::WalletError;
use crate::net::asset::Asset;
use crate::net::pool::{ACTIVE, RPCS};
use crate::net::tor::Tor;
use crate::wallet::{check_deposit, prepare_deposit, read_pool_state_checked, DepositRequest};

const ONE_BY_ONE: &str = "This pool takes these deposits one by one. Deposit each size alone.";

impl Wallet {
    /// The requests, or the pool's refusal in a sentence.
    pub(super) fn fresh_requests(
        &self,
        tor: &Tor,
        from: &[u8; 20],
        asset: Asset,
        sizes: &[u64],
    ) -> Result<Result<Vec<DepositRequest>, &'static str>, WalletError> {
        let to = self.with(|s| s.address())?;
        let state = read_pool_state_checked(tor, &RPCS, &ACTIVE, from, asset.id)?;
        if state.scale != asset.scale || state.beta_mode {
            return Ok(Err(ONE_BY_ONE));
        }
        let mut requests = Vec::with_capacity(sizes.len());
        for size in sizes {
            let base = u128::from(*size).checked_mul(asset.scale).ok_or(WalletError::Amount)?;
            if let Err(refusal) = check_deposit(&state, base) {
                return Ok(Err(super::super::ticket::sentence(refusal)));
            }
            requests.push(prepare_deposit(&to, base, asset.id, state.fee_bps, state.scale)?);
        }
        Ok(Ok(requests))
    }
}
