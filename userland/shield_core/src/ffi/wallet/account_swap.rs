//! Swaps from the public account. A swap review is held under an id like a send
//! review and confirmed through the same call, so the owner's yes, the chain
//! checks and the nonce record are shared.

use super::account_view::VALID_FOR;
use super::swap_view::{refused, shown};
use super::{Pending, Wallet};
use crate::error::{NetError, WalletError};
use crate::evm::swap::{review_swap, Swap, SwapChecked};
use crate::evm::{self, Network};
use crate::ffi::account_types::PublicSwap;
use crate::ffi::amount_read::parse_units;
use crate::net::asset::Coin;
use std::time::Instant;

impl Wallet {
    /// Work out a swap of `amount` of `from` for `to` on `network`, allowing
    /// `slippage_bps` below the quote, held for `confirm_public_send`.
    pub fn review_public_swap(
        &self,
        network: Network,
        from: Coin,
        to: Coin,
        amount: String,
        slippage_bps: u16,
    ) -> Result<PublicSwap, WalletError> {
        let amount = parse_units(&amount, from.decimals())?;
        let swap = Swap { from, to, amount, slippage_bps };
        let account = self.with(|s| s.evm().address())?;
        let sends = evm::nonce::recorded(&self.paths().account, network, &account);
        let tor = self.tor()?;
        let review = match review_swap(tor.as_ref(), network, &account, &swap, &sends)? {
            SwapChecked::Ready(review) => review,
            SwapChecked::Refused(why) => return Ok(refused(network, &swap, why)),
        };
        let id = self.next_review_id();
        let view =
            shown(id, network, &swap, &review, u32::try_from(VALID_FOR.as_secs()).unwrap_or(0));
        let pending =
            Pending { id, network, from: account, tx: review.tx, made: Instant::now(), note: None };
        *self.pending.lock().map_err(|_| NetError::Transport)? = Some(pending);
        Ok(view)
    }
}

/// Whether swaps run on `network`: mainnet, where the pools are.
pub fn network_has_swaps(network: Network) -> bool {
    network.chain().swaps.is_some()
}
