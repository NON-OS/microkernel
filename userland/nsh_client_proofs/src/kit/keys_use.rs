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

//! Signing with the fleet's keys, and the forms a proof needs them in.

use super::keys::{user, Hybrid};
use super::rng::Det;
use nonos_mldsa65::SecretKey;
use nsh_client::{HybridKey, UserKey};
use nsh_crypto::{key_digest, Secret};

impl Hybrid {
    /// Both halves over `msg`: 64 bytes of Ed25519, then 3309 of ML-DSA-65.
    pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
        let mut out = nonos_ed25519::sign(&self.ed, msg).to_bytes().to_vec();
        let ml = nonos_mldsa65::sign(&self.ml, msg, &mut |b| Det::new(9).fill(b)).unwrap();
        out.extend_from_slice(&ml);
        out
    }

    pub fn public(&self) -> HybridKey {
        HybridKey { ed: self.ed.public, ml: Box::new(self.ml_public) }
    }

    pub fn public_bytes(&self) -> Vec<u8> {
        [&self.ed.public[..], &self.ml_public[..]].concat()
    }

    pub fn id(&self) -> [u8; 32] {
        self.public().key_id()
    }

    pub fn digest(&self) -> [u8; 64] {
        key_digest(&self.ed.public, &self.ml_public)
    }
}

/// The user's key as the client holds it.
pub fn user_key() -> UserKey {
    let u = user();
    UserKey::from_parts(
        Secret::new([0x43; 32]),
        SecretKey::from_bytes(u.ml.as_bytes()),
        &u.ml_public,
    )
}
