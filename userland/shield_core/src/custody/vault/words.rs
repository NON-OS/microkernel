//! The words a version 2 vault holds, read back for the owner. A version 1 vault and a wallet from
//! a private key hold none.

use super::Vault;
use crate::custody::format::Kind;
use crate::custody::guard::HardwareGuard;
use crate::custody::phrase::Phrase;
use crate::error::CustodyError;
use nonos_hd::bip39::MAX_WORDS;

impl Vault {
    /// The words the seed came from, or none for a version 1 vault, which never held them.
    pub fn load_words(&self, guard: &dyn HardwareGuard) -> Result<Option<Phrase>, CustodyError> {
        let (kind, plain) = self.open_plain(guard)?;
        if kind != Kind::SeedAndWords {
            return Ok(None);
        }
        let count = usize::from(*plain.get(64).ok_or(CustodyError::SealAuth)?);
        let mut indices = [0u16; MAX_WORDS];
        let words = plain.get(65..).ok_or(CustodyError::SealAuth)?;
        for (slot, pair) in indices.iter_mut().zip(words.chunks_exact(2)) {
            *slot = u16::from_le_bytes(pair.try_into().map_err(|_| CustodyError::SealAuth)?);
        }
        // Authenticated, yet checked: words that fail their checksum are never shown as a backup.
        Phrase::new(indices, count).map(Some).ok_or(CustodyError::SealAuth)
    }
}
