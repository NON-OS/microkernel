//! How many accounts the wallet shows and which is active, in a file beside the nonce records.
//! It holds two small numbers and no key. A missing or unreadable file reads as one account.

use crate::error::{StoreError, WalletError};
use std::path::{Path, PathBuf};

/// The most accounts one wallet holds, so a damaged file cannot ask for a million derivations.
pub const MAX_ACCOUNTS: u32 = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Book {
    pub count: u32,
    pub active: u32,
}

fn file(dir: &Path) -> PathBuf {
    dir.join("accounts")
}

impl Book {
    pub(crate) fn read(dir: &Path) -> Book {
        let text = std::fs::read_to_string(file(dir)).unwrap_or_default();
        let mut parts = text.split_whitespace().map(str::parse::<u32>);
        let count = match parts.next() {
            Some(Ok(n)) if (1..=MAX_ACCOUNTS).contains(&n) => n,
            _ => 1,
        };
        let active = match parts.next() {
            Some(Ok(n)) if n < count => n,
            _ => 0,
        };
        Book { count, active }
    }

    /// Written to a side file and renamed over the old one, so a crash leaves one or the other.
    pub(crate) fn write(self, dir: &Path) -> Result<(), WalletError> {
        let io = |_| WalletError::Store { source: StoreError::Io };
        std::fs::create_dir_all(dir).map_err(io)?;
        let side = file(dir).with_extension("new");
        std::fs::write(&side, format!("{} {}\n", self.count, self.active)).map_err(io)?;
        std::fs::rename(side, file(dir)).map_err(io)
    }
}
