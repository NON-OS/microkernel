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

pub fn key(k: K) -> &'static TestKey {
    static KEYS: OnceLock<Vec<TestKey>> = OnceLock::new();
    &KEYS.get_or_init(|| (0..8).map(make).collect())[k as usize]
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
