//! What a closed session leaves behind, which is nothing.
//!
//! The spend secret is wiped with volatile writes when the account drops, and
//! the Debug prints only that it is redacted. Deriving Debug here would print four field
//! elements that are the wallet.

use super::account::Account;
use nonos_stark::field::Fp;

impl Drop for Account {
    fn drop(&mut self) {
        for w in self.sk.iter_mut() {
            // SAFETY: the spend secret must not survive the session in a stack
            // or heap slot the optimiser decided to keep.
            unsafe { core::ptr::write_volatile(w, Fp::ZERO) };
        }
    }
}

impl core::fmt::Debug for Account {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Account(redacted)")
    }
}
