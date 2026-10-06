//! Writing the vault. An existing vault is never overwritten: it may be the only copy of a seed.
//! The file key is drawn fresh and wiped from this process before the blob is written.

use super::{Vault, NONCE};
use crate::custody::format::{encode, Kind};
use crate::custody::guard::HardwareGuard;
use crate::custody::phrase::Phrase;
use crate::custody::secret::Seed;
use crate::entropy::fill;
use crate::error::CustodyError;
use nonos_hd::bip39::MAX_WORDS;
use nonos_seal::{seal, TAG_LEN};
use zeroize::{Zeroize, Zeroizing};

impl Vault {
    /// Seal `seed` alone, the version 1 vault, refusing if one already exists.
    pub fn store(&self, seed: &Seed, guard: &dyn HardwareGuard) -> Result<(), CustodyError> {
        self.seal_and_write(Kind::Seed, seed.bytes(), guard)
    }

    /// Seal `seed` with the words it came from, so the owner can be shown them again.
    pub fn store_with_words(
        &self,
        seed: &Seed,
        phrase: &Phrase,
        guard: &dyn HardwareGuard,
    ) -> Result<(), CustodyError> {
        let mut plain = Zeroizing::new(seed.bytes().to_vec());
        let count = u8::try_from(phrase.indices().len()).map_err(|_| CustodyError::Mnemonic)?;
        plain.push(count);
        let mut slots = [0u16; MAX_WORDS];
        for (slot, index) in slots.iter_mut().zip(phrase.indices()) {
            *slot = *index;
        }
        for index in slots {
            plain.extend_from_slice(&index.to_le_bytes());
        }
        self.seal_and_write(Kind::SeedAndWords, &plain, guard)
    }

    /// Seal a private key brought in, for a wallet with no words.
    pub fn store_key(&self, key: &[u8; 32], guard: &dyn HardwareGuard) -> Result<(), CustodyError> {
        self.seal_and_write(Kind::Key, key, guard)
    }

    fn seal_and_write(
        &self,
        kind: Kind,
        plain: &[u8],
        guard: &dyn HardwareGuard,
    ) -> Result<(), CustodyError> {
        if self.exists() {
            return Err(CustodyError::VaultPresent);
        }
        let mut key = [0u8; 32];
        fill(&mut key)?;
        let mut sealed = vec![0u8; plain.len().saturating_add(TAG_LEN)];
        let sealed_len = seal(&key, &NONCE, kind.aad(), plain, &mut sealed);
        let wrapped = guard.wrap(key.to_vec());
        key.zeroize();
        if sealed_len != Ok(sealed.len()) {
            return Err(CustodyError::SealAuth);
        }
        let blob = encode(kind, &wrapped?, &sealed)?;
        self.write_blob(&blob)
    }
}
