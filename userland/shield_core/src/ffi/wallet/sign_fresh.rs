//! SIGN_FROM_FRESH for the shell (v1.4.51): a refund or a dispute signed by
//! a fresh address this wallet handed out. Every check runs before the vault
//! is opened again for the seed; the key is derived for this signature and
//! dropped, zeroed, with the seed before this returns.

use super::Wallet;
use crate::custody::Vault;
use crate::error::WalletError;
use crate::evm::tx::Signed;
use crate::wallet::fresh::{checked, handed_index, sign_at, FreshRefusal, FreshSign};
use nonos_nox_config::{address, Contract};

impl Wallet {
    /// The signed transaction, or the refusal by name.
    pub fn sign_from_fresh(
        &self,
        r: &FreshSign,
    ) -> Result<Result<Signed, FreshRefusal>, WalletError> {
        let handed = handed_index(&self.paths().account, &r.address);
        let escrow = |chain_id| address(chain_id, Contract::Escrow).ok();
        let (index, tx) = match checked(r, handed, escrow) {
            Ok(ready) => ready,
            Err(why) => return Ok(Err(why)),
        };
        let seed = Vault::at(self.paths().vault.clone()).load(self.guard())?;
        let signed = sign_at(seed.bytes(), index, &r.address, &tx);
        drop(seed);
        Ok(signed.ok_or(FreshRefusal::UnknownAddress))
    }
}
