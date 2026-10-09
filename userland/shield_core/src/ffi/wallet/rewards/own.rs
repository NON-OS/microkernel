//! A rewards link whose mainnet side is an account of this wallet: the key of that account signs
//! the message here, and the active account sends it.

use super::super::Wallet;
use super::send::refused;
use super::view::{sentence, RewardsReview};
use crate::error::WalletError;
use crate::wallet::rewards::{digest, link_calldata, may_link, read_link, sign_link};

impl Wallet {
    /// A link of account `account` of this wallet, as the mainnet side, to the active account.
    pub fn review_rewards_link_own(&self, account: u32) -> Result<RewardsReview, WalletError> {
        let (key, mainnet, testnet) = self.with(|s| {
            let owner = s.evm_at(account);
            (owner.and_then(|o| o.signing_key()), owner.map(|o| o.address()), s.evm().address())
        })?;
        let mainnet = mainnet.ok_or(WalletError::NoSuchAccount)?;
        let tor = self.tor()?;
        let state = read_link(&tor, &testnet, &mainnet)?;
        if let Err(why) = may_link(&state, &mainnet, &testnet) {
            return Ok(refused(&mainnet, &testnet, sentence(why)));
        }
        let key = key.ok_or(WalletError::NoSuchAccount)?;
        let signature = sign_link(&key, &digest(&mainnet, &testnet, state.nonce))
            .ok_or(WalletError::NoSuchAccount)?;
        let call = link_calldata(&mainnet, &signature, false);
        self.review_call((&tor, &state), (mainnet, testnet), &call)
    }
}
