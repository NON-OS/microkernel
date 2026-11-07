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

//! The test keys. Each is derived from a fixed seed inside the test, so
//! every run signs the same bytes; none comes from any keys directory, and
//! none is used anywhere but here.

use super::stream::{make, stream};
use nonos_ed25519::KeyPair;
use nonos_hash::{sha256, sha512};
use nonos_mldsa65::{SecretKey, PUBLIC_LEN};
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum K {
    /// The role authority the host pins, and the default issuer.
    AuthA,
    AuthB,
    Appr1,
    Appr2,
    Appr3,
    User,
    OtherUser,
    Stranger,
}

pub struct TestKey {
    pub ed: KeyPair,
    pub ml_pk: [u8; PUBLIC_LEN],
    pub ml_sk: SecretKey,
}

static KEYS: OnceLock<Vec<TestKey>> = OnceLock::new();

pub fn key(k: K) -> &'static TestKey {
    &KEYS.get_or_init(|| (0..8).map(make).collect())[k as usize]
}

/// Mixed into every key's seed once set. The proofs never set it, so their
/// keys are the fixed ones; the devkit sets it from the OS generator.
static SPACE: OnceLock<[u8; 32]> = OnceLock::new();

/// Make every key from `space`. Refused once any key is made, or if a space
/// is already set, so no key is ever made from two spaces.
pub fn reseed(space: [u8; 32]) -> bool {
    KEYS.get().is_none() && SPACE.set(space).is_ok()
}

/// The space keys are made from, if one is set.
pub fn space() -> &'static [u8] {
    SPACE.get().map_or(&[], |s| &s[..])
}

impl TestKey {
    /// The hybrid public key: Ed25519, then ML-DSA-65.
    pub fn public(&self) -> Vec<u8> {
        [&self.ed.public[..], &self.ml_pk[..]].concat()
    }

    pub fn id(&self) -> [u8; 32] {
        sha256(&self.public())
    }

    pub fn subject(&self) -> [u8; 64] {
        sha512(&self.public())
    }

    /// A hybrid signature over `digest`: Ed25519, then ML-DSA-65.
    pub fn sign(&self, digest: &[u8; 64]) -> Vec<u8> {
        let ed = nonos_ed25519::sign(&self.ed, digest).to_bytes();
        let ml = nonos_mldsa65::sign(&self.ml_sk, digest, &mut stream(digest)).expect("sign");
        [&ed[..], &ml[..]].concat()
    }
}
