//! A link or an unlink in the rewards registry, sent from the active account on Sepolia. The pair
//! is checked against the registry and the signature against the mainnet address before review,
//! and the call is then reviewed and held like a deposit, for `confirm_public_send`.

use super::super::Wallet;
use super::send::refused;
use super::view::{sentence, RewardsReview};
use crate::error::WalletError;
use crate::ffi::evm::parse_evm_address;
use crate::wallet::rewards::{accepts, digest, link_calldata, may_link, may_unlink};
use crate::wallet::rewards::{parse_signature, read_link, signer, unlink_calldata};

impl Wallet {
    /// A link of `mainnet` to the active account, with the signature its own wallet made.
    pub fn review_rewards_link(
        &self,
        mainnet: String,
        signature: String,
    ) -> Result<RewardsReview, WalletError> {
        let mainnet = parse_evm_address(&mainnet)?;
        let testnet = self.with(|s| s.evm().address())?;
        let Some(signature) = parse_signature(&signature) else {
            return Ok(refused(&mainnet, &testnet, "That is not a signature the registry takes."));
        };
        let tor = self.tor()?;
        let state = read_link(&tor, &testnet, &mainnet)?;
        if let Err(why) = may_link(&state, &mainnet, &testnet) {
            return Ok(refused(&mainnet, &testnet, sentence(why)));
        }
        let message = digest(&mainnet, &testnet, state.nonce);
        let contract_wallet = match signer(&message, &signature) {
            Some(by) if by == mainnet => false,
            _ if accepts(&tor, &mainnet, &message, &signature)? => true,
            _ => {
                let why = "That signature is not from that mainnet address for this account now.";
                return Ok(refused(&mainnet, &testnet, why));
            }
        };
        let call = link_calldata(&mainnet, &signature, contract_wallet);
        self.review_call((&tor, &state), (mainnet, testnet), &call)
    }

    /// The end of the link of the active account, from the next epoch.
    pub fn review_rewards_unlink(&self) -> Result<RewardsReview, WalletError> {
        let testnet = self.with(|s| s.evm().address())?;
        let tor = self.tor()?;
        let state = read_link(&tor, &testnet, &testnet)?;
        let mainnet = state.linked_to.unwrap_or_default();
        if let Err(why) = may_unlink(&state) {
            return Ok(refused(&mainnet, &testnet, sentence(why)));
        }
        self.review_call((&tor, &state), (mainnet, testnet), &unlink_calldata())
    }
}
