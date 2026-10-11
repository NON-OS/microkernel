//! Where a withdrawal goes. A withdrawal is public, so it pays an address that has never been
//! used: the public address of this wallet's next account, which the words restore. An address
//! typed in is checked for any past use first, so the screen can warn before it is paid.

use super::Wallet;
use crate::error::WalletError;
use crate::evm::{checksummed, used, Network};
use crate::ffi::evm::parse_evm_address;

impl Wallet {
    /// A fresh address of this wallet for a withdrawal, or none for a wallet from a private key.
    /// It is the account at index `count`, kept as handed out so SIGN_FROM_FRESH may sign
    /// a refund or a dispute from it (v1.4.51).
    pub fn fresh_withdrawal_address(&self) -> Result<Option<String>, WalletError> {
        let Some((index, a)) = self.with(|s| s.next_public.map(|a| (s.count(), a)))? else {
            return Ok(None);
        };
        crate::wallet::fresh::keep_handed(&self.paths().account, index, &a)?;
        Ok(Some(checksummed(&a)))
    }

    /// Whether `address` has sent a transaction or holds anything on Sepolia, read over Tor.
    pub fn address_used(&self, address: String) -> Result<bool, WalletError> {
        let address = parse_evm_address(&address)?;
        Ok(used(self.tor()?.as_ref(), Network::Sepolia, &address)?)
    }
}
