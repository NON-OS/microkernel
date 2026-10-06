//! Sends from the public account, reviewed in full before one is signed. A review
//! holds the exact transaction the screen described, briefly, under an id. Confirming
//! signs that transaction only, and an unknown, used or stale id is refused.

use super::account::parse_recipient;
use super::account_view::{refusal, view};
use super::{Pending, Wallet};
use crate::error::{NetError, WalletError};
use crate::evm::{self, Checked, Network, Order};
use crate::ffi::account_types::PublicReview;
use crate::ffi::amount_read::parse_units;
use crate::net::asset::Coin;
use std::time::Instant;

impl Wallet {
    /// Work out a send of `amount` of `coin` to `to` on `network`, and hold it.
    pub fn review_public_send(
        &self,
        network: Network,
        coin: Coin,
        to: String,
        amount: String,
    ) -> Result<PublicReview, WalletError> {
        let amount = parse_units(&amount, coin.decimals())?;
        let order = Order { coin, to: parse_recipient(&to)?, amount };
        let from = self.with(|s| s.evm().address())?;
        let sends = evm::nonce::recorded(&self.paths().account, network, &from);
        let review = match evm::review(self.tor()?.as_ref(), network, &from, &order, &sends)? {
            Checked::Ready(review) => review,
            Checked::Refused(why) => return Ok(refusal(network, &from, &order, why)),
        };
        let id = self.next_review_id();
        let shown = view(id, network, &from, &order, &review);
        let pending =
            Pending { id, network, from, tx: review.tx, made: Instant::now(), note: None };
        *self.pending.lock().map_err(|_| NetError::Transport)? = Some(pending);
        Ok(shown)
    }
}
