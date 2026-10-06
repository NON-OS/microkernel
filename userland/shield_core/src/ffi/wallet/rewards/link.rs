//! The rewards link screen: where the active account stands in the registry on Sepolia, and the
//! typed data a wallet that holds a mainnet address signs to link it to the active account.

use super::super::Wallet;
use super::view::{link_view, RewardsLink};
use crate::error::WalletError;
use crate::ffi::evm::parse_evm_address;
use crate::wallet::rewards::{read_link, typed_json};

impl Wallet {
    /// The link of the active account, read over Tor.
    pub fn rewards_link(&self) -> Result<RewardsLink, WalletError> {
        let testnet = self.with(|s| s.evm().address())?;
        let tor = self.tor()?;
        let own = read_link(&tor, &testnet, &testnet)?;
        let state = match own.linked_to {
            Some(mainnet) if mainnet != testnet => read_link(&tor, &testnet, &mainnet)?,
            _ => own,
        };
        Ok(link_view(&testnet, &state))
    }

    /// The `eth_signTypedData_v4` data that links `mainnet` to the active account at its nonce now.
    pub fn rewards_message(&self, mainnet: String) -> Result<String, WalletError> {
        let mainnet = parse_evm_address(&mainnet)?;
        let testnet = self.with(|s| s.evm().address())?;
        let state = read_link(self.tor()?.as_ref(), &testnet, &mainnet)?;
        Ok(typed_json(&mainnet, &testnet, state.nonce))
    }
}
