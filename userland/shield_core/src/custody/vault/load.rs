//! Opening the vault. The keystore unwraps first, so a failed authentication never reaches the
//! seal. The file key and the plaintext are wiped on every path, and a wrong key length is refused.

use super::{Vault, NONCE};
use crate::custody::format::{decode, Kind};
use crate::custody::guard::HardwareGuard;
use crate::custody::imported::seed_of_key;
use crate::custody::secret::Seed;
use crate::error::CustodyError;
use nonos_seal::{open, TAG_LEN};
use zeroize::{Zeroize, Zeroizing};

/// What an opened vault holds: always a seed, and the key for a wallet from a private key.
pub struct Opened {
    pub seed: Seed,
    pub key: Option<Zeroizing<[u8; 32]>>,
}

impl Vault {
    /// Read the vault and open the seal.
    pub fn load(&self, guard: &dyn HardwareGuard) -> Result<Seed, CustodyError> {
        Ok(self.open_all(guard)?.seed)
    }

    /// The seed, and for a wallet from a private key the key itself, from one unwrap.
    pub fn open_all(&self, guard: &dyn HardwareGuard) -> Result<Opened, CustodyError> {
        let (kind, plain) = self.open_plain(guard)?;
        if kind == Kind::Key {
            let mut key = Zeroizing::new([0u8; 32]);
            key.copy_from_slice(plain.get(..32).ok_or(CustodyError::SealAuth)?);
            return Ok(Opened { seed: seed_of_key(&key), key: Some(key) });
        }
        let mut seed = [0u8; 64];
        seed.copy_from_slice(plain.get(..64).ok_or(CustodyError::SealAuth)?);
        let out = Seed::new(seed);
        seed.zeroize();
        Ok(Opened { seed: out, key: None })
    }

    pub(super) fn open_plain(
        &self,
        guard: &dyn HardwareGuard,
    ) -> Result<(Kind, Zeroizing<Vec<u8>>), CustodyError> {
        let blob = self.read_blob()?;
        let (kind, wrapped, sealed) = decode(&blob)?;
        let mut key = [0u8; 32];
        let mut unwrapped = guard.unwrap_key(wrapped)?;
        let wrong_length = unwrapped.len() != key.len();
        if !wrong_length {
            key.copy_from_slice(&unwrapped);
        }
        unwrapped.zeroize();
        if wrong_length {
            return Err(CustodyError::Guard);
        }
        let mut plain = Zeroizing::new(vec![0u8; sealed.len().saturating_sub(TAG_LEN)]);
        let opened = open(&key, &NONCE, kind.aad(), &sealed, &mut plain);
        key.zeroize();
        if opened != Ok(plain.len()) {
            return Err(CustodyError::SealAuth);
        }
        Ok((kind, plain))
    }
}
