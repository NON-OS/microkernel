// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The fleet authority's hybrid key, Ed25519 and ML-DSA-65, and the file
//! that holds it. The secret file is `NSHA`, version 1, two zero bytes,
//! the Ed25519 seed, the ML-DSA-65 public key, then its secret key. A
//! public key, an authority's or a host's, is the Ed25519 key then the
//! ML-DSA-65 key, 1984 bytes, the order `key_digest` and the key id use.

use nonos_ed25519::KeyPair;
use nonos_mldsa65::{SecretKey, PUBLIC_LEN, SECRET_LEN};

use crate::Error;

mod file;
mod wipe;

pub use wipe::wipe;

pub const PUBLIC_FILE_LEN: usize = 32 + PUBLIC_LEN;
pub const SECRET_FILE_LEN: usize = 8 + 32 + PUBLIC_LEN + SECRET_LEN;

pub struct AuthorityKey {
    pub(crate) ed: KeyPair,
    pub(crate) ml_public: Box<[u8; PUBLIC_LEN]>,
    pub(crate) ml_secret: SecretKey,
}

impl AuthorityKey {
    /// A new key pair from the system's randomness.
    pub fn generate() -> Result<Self, Error> {
        let mut seed = [0u8; 32];
        if !crate::rand::fill(&mut seed) {
            return Err(Error::Randomness);
        }
        let ed = KeyPair::from_seed(seed);
        seed.fill(0);
        let (pk, ml_secret) =
            nonos_mldsa65::keypair(&mut crate::rand::source()).map_err(|_| Error::Randomness)?;
        Ok(AuthorityKey { ed, ml_public: Box::new(pk), ml_secret })
    }

    pub fn public(&self) -> Vec<u8> {
        [&self.ed.public[..], &self.ml_public[..]].concat()
    }

    /// The key id certificates name: SHA-256 of the public key.
    pub fn id(&self) -> [u8; 32] {
        nonos_hash::sha256(&self.public())
    }

    /// Both halves over `digest`, Ed25519 then ML-DSA-65, with the system's
    /// randomness for ML-DSA; nothing if either half cannot be made.
    pub fn sign(&self, digest: &[u8; 64]) -> Result<Vec<u8>, Error> {
        let ed = nonos_ed25519::sign(&self.ed, digest).to_bytes();
        let ml = nonos_mldsa65::sign(&self.ml_secret, digest, &mut crate::rand::source())
            .map_err(|_| Error::Randomness)?;
        Ok([&ed[..], &ml[..]].concat())
    }
}
