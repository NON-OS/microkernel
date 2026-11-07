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

//! The host keys in memory: the signing key nsh_server's `HostKey` builds
//! from the plaintext, the public keys nsh.front asks for, and their key
//! id. The plaintext is wiped as soon as the key is built.

use alloc::boxed::Box;

use nonos_mldsa65::SecretKey;
use nsh_crypto::Secret;
use nsh_front_wire::{DIGEST_LEN, ED_SIG_LEN, KEYS_LEN, SIG_LEN};
use nsh_server::{HostKey, HostSigner};

use super::layout::{split, ED_SEED_LEN, PLAIN_LEN};
use crate::rng;

pub struct Held {
    key: HostKey,
    public: Box<[u8; KEYS_LEN]>,
    id: [u8; 32],
}

impl Held {
    /// The keys `plain` holds. `plain` is wiped.
    pub fn from_plain(plain: &mut [u8; PLAIN_LEN]) -> Held {
        let (seed, secret, ml_public) = split(plain);
        let mut seed: [u8; ED_SEED_LEN] = *seed;
        let key =
            HostKey::from_parts(Secret::take(&mut seed), SecretKey::from_bytes(secret), ml_public);
        let mut public = Box::new([0u8; KEYS_LEN]);
        public[..32].copy_from_slice(key.ed_public());
        public[32..].copy_from_slice(key.ml_public());
        nonos_hash::wipe(plain);
        let id = nonos_hash::sha256(&public[..]);
        Held { key, public, id }
    }

    /// The Ed25519 key, then the ML-DSA-65 key.
    pub fn public(&self) -> &[u8; KEYS_LEN] {
        &self.public
    }

    /// SHA-256 of the public keys: the key id nsh-enrol names them by.
    pub fn id(&self) -> &[u8; 32] {
        &self.id
    }

    /// Both signatures over `digest`, Ed25519 then ML-DSA-65, or `None` when
    /// the generator failed and nothing was signed.
    pub fn sign(&self, digest: &[u8; DIGEST_LEN]) -> Option<Box<[u8; SIG_LEN]>> {
        let sig = self.key.sign(digest, &mut |b: &mut [u8]| rng::fill(b))?;
        let mut out = Box::new([0u8; SIG_LEN]);
        out[..ED_SIG_LEN].copy_from_slice(&sig.ed);
        out[ED_SIG_LEN..].copy_from_slice(&sig.ml[..]);
        Some(out)
    }
}
