//! A fresh address's signature (v1.4.51): its key derived from the seed for
//! this signature only, checked to be the address asked for, and zeroed
//! with the account when it drops.

use crate::evm::tx::{Eip1559, Signed};
use crate::evm::EvmAccount;

/// `tx` signed by account `index` of `seed`, only if that account is
/// `address`; None otherwise.
pub fn sign_at(seed: &[u8; 64], index: u32, address: &[u8; 20], tx: &Eip1559) -> Option<Signed> {
    let account = EvmAccount::at(seed, index).filter(|a| a.address() == *address)?;
    tx.sign(&account.signing_key()?)
}
