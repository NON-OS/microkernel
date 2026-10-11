//! OP_FRESH_ADDRESS mode 1 for the shell: one address per
//! order or channel. The shell reads the addresses ahead with the account
//! count after each job, and answers mode 1 from them without the session,
//! which a proof may hold for minutes.

use super::Wallet;
use crate::error::WalletError;
use crate::evm::checksummed;
use crate::wallet::fresh::fresh_once;

impl Wallet {
    /// The account count and the addresses mode 1 may hand out, empty for a
    /// wallet from a private key.
    pub fn fresh_ahead(&self) -> Result<(u32, Vec<(u32, [u8; 20])>), WalletError> {
        self.with(|s| (s.count(), s.ahead.clone()))
    }

    /// Mode 1: the next address no order or channel shares, kept as handed
    /// before it is answered; None once those derived at unlock are used.
    pub fn fresh_once(
        &self,
        count: u32,
        ahead: &[(u32, [u8; 20])],
    ) -> Result<Option<String>, WalletError> {
        Ok(fresh_once(&self.paths().account, count, ahead)?.map(|a| checksummed(&a)))
    }

    /// Mode 2: every address handed out, index and lowercase hex, without the session.
    pub fn fresh_handed(&self) -> Vec<(u32, String)> {
        crate::wallet::fresh::handed_all(&self.paths().account)
    }
}
