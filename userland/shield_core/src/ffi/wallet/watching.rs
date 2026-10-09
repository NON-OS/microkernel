//! View-only accounts as the shells see them: add one from a pasted view key, list them with
//! what each can see, and forget one. None of them can send, and none becomes the active account.

use super::Wallet;
use crate::error::WalletError;
use crate::ffi::WatchedSummary;
use crate::net::pool::ACTIVE;

impl Wallet {
    /// Add a view-only account from a `noxivk1` or `noxfvk1` key. Its notes appear after a sync.
    pub fn import_view_key(&self, text: String) -> Result<u32, WalletError> {
        self.with_mut(|s| s.watching.add(&text))
    }

    pub fn watched_accounts(&self) -> Result<Vec<WatchedSummary>, WalletError> {
        self.with(|s| {
            (0u32..)
                .zip(s.watching.list.iter())
                .map(|(index, w)| WatchedSummary {
                    index,
                    kind: w.key.kind,
                    balances: ACTIVE.assets.iter().map(|a| w.state.balance(a)).collect(),
                })
                .collect()
        })
    }

    /// Forget view-only account `index` on this device. The key itself stays valid wherever else
    /// it was shared: a view key cannot be revoked.
    pub fn forget_view_key(&self, index: u32) -> Result<(), WalletError> {
        self.with_mut(|s| s.watching.forget(index))
    }
}
