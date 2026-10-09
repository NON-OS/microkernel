//! The registry call of a link or an unlink, reviewed as the sender on Sepolia and held under an id
//! for the owner to confirm, as a deposit is.

use super::super::account_view::VALID_FOR;
use super::super::{Pending, Wallet};
use super::view::RewardsReview;
use crate::error::{NetError, WalletError};
use crate::evm::shield::{review_shield, ShieldChecked, ShieldOrder};
use crate::evm::{self, checksummed, Network};
use crate::net::tor::Tor;
use crate::wallet::rewards::{counts_from, LinkState, REGISTRY_BYTES};
use std::time::Instant;

impl Wallet {
    pub(super) fn review_call(
        &self,
        (tor, state): (&Tor, &LinkState),
        (mainnet, testnet): ([u8; 20], [u8; 20]),
        call: &[u8],
    ) -> Result<RewardsReview, WalletError> {
        let order = ShieldOrder {
            pool: REGISTRY_BYTES,
            token: None,
            amount: 0,
            approve_data: &[],
            deposit_data: call,
            refused_as: "The registry would not take this now. Nothing was sent.",
        };
        let sends = evm::nonce::recorded(&self.paths().account, Network::Sepolia, &testnet);
        let review = match review_shield(tor, &testnet, &order, &sends)? {
            ShieldChecked::Ready(review) => review,
            ShieldChecked::Refused(why) => return Ok(refused(&mainnet, &testnet, why)),
        };
        let id = self.next_review_id();
        let shown = RewardsReview {
            id,
            refusal: None,
            counts_from_epoch: counts_from(state).0,
            max_network_fee: crate::evm::units::ether(review.max_network_fee),
            valid_for_seconds: u32::try_from(VALID_FOR.as_secs()).unwrap_or(0),
            ..refused(&mainnet, &testnet, "")
        };
        let held = Pending {
            id,
            network: Network::Sepolia,
            from: testnet,
            tx: review.tx,
            made: Instant::now(),
            note: None,
        };
        *self.pending.lock().map_err(|_| NetError::Transport)? = Some(held);
        Ok(shown)
    }
}

/// A link or an unlink that was not reviewed, and why.
pub(super) fn refused(mainnet: &[u8; 20], testnet: &[u8; 20], why: &str) -> RewardsReview {
    RewardsReview {
        id: 0,
        refusal: (!why.is_empty()).then(|| why.to_string()),
        mainnet: checksummed(mainnet),
        testnet: checksummed(testnet),
        counts_from_epoch: 0,
        max_network_fee: String::new(),
        valid_for_seconds: 0,
    }
}
