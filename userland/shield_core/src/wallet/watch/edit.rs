//! Adding a view key the user pasted, and forgetting one. The file is rewritten whole each time.

use super::Watching;
use crate::error::{StoreError, WalletError};
use zeroize::Zeroizing;

impl Watching {
    /// Add a view key the user pasted. The same key twice is one account.
    pub(crate) fn add(&mut self, text: &str) -> Result<u32, WalletError> {
        let entry =
            self.entry(Zeroizing::new(text.split_whitespace().collect::<String>().to_lowercase()))?;
        if let Some(i) = self.list.iter().position(|w| w.text == entry.text) {
            return u32::try_from(i).map_err(|_| WalletError::NoSuchAccount);
        }
        self.list.push(entry);
        self.save()?;
        u32::try_from(self.list.len().saturating_sub(1)).map_err(|_| WalletError::NoSuchAccount)
    }

    /// Forget view-only account `index`, its log with it.
    pub(crate) fn forget(&mut self, index: u32) -> Result<(), WalletError> {
        let i = usize::try_from(index).map_err(|_| WalletError::NoSuchAccount)?;
        if i >= self.list.len() {
            return Err(WalletError::NoSuchAccount);
        }
        let gone = self.list.remove(i);
        self.save()?;
        match std::fs::remove_file(self.log_path(&gone.text)) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(StoreError::Io.into()),
            _ => Ok(()),
        }
    }
}
