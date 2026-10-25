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

//! A hybrid key as the role table holds it, and the check of a hybrid
//! signature: Ed25519 and ML-DSA-65 over the same digest. Both halves are
//! always checked, and both must verify.

use nonos_ed25519::Signature;
use nonos_hash::Sha256;
use nsh_wire::keys::{HybridPublic, HybridSig, DIGEST_LEN, KEY_ID_LEN};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key<'a> {
    /// SHA-256 of the key, as a grant names its issuer and approvers.
    pub id: [u8; KEY_ID_LEN],
    /// SHA-512 of the key, as a grant names its subject.
    pub subject: [u8; DIGEST_LEN],
    pub public: HybridPublic<'a>,
}

impl<'a> Key<'a> {
    pub fn new(public: HybridPublic<'a>) -> Self {
        let subject = nsh_crypto::key_digest(public.ed, public.ml);
        Key { id: key_id(&public), subject, public }
    }
}

/// A key id: SHA-256 of the Ed25519 key, then the ML-DSA-65 key.
pub fn key_id(public: &HybridPublic<'_>) -> [u8; KEY_ID_LEN] {
    let mut h = Sha256::new();
    h.update(public.ed);
    h.update(public.ml);
    h.finalize()
}

/// Both signatures verify over `digest` under `key`.
pub fn verify(key: &HybridPublic<'_>, digest: &[u8; DIGEST_LEN], sig: &HybridSig<'_>) -> bool {
    let ed = nonos_ed25519::verify(key.ed, digest, &Signature::from_bytes(sig.ed));
    let ml = nonos_mldsa65::verify(key.ml, digest, sig.ml);
    ed & ml
}
