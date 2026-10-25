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

//! What the client holds about its fleet, from `nsh-client-policy/1`: the
//! fleet id, the enrolment authority's hybrid keys, and the enrolments
//! withdrawn before their certificates ran out. The release values the
//! attestation check needs are the `Attester`'s. Checking the policy
//! file's own signature is the caller's, before it builds a `Policy`.

use alloc::boxed::Box;
use alloc::vec::Vec;
use nonos_hash::Sha256;
use nsh_wire::keys::{ED_PUBLIC_LEN, HYBRID_PUBLIC_LEN, KEY_ID_LEN, ML_PUBLIC_LEN};

/// A hybrid public key: Ed25519, then ML-DSA-65.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridKey {
    pub ed: [u8; ED_PUBLIC_LEN],
    pub ml: Box<[u8; ML_PUBLIC_LEN]>,
}

impl HybridKey {
    /// A key laid out as the wire lays it: 32 bytes, then 1952.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != HYBRID_PUBLIC_LEN {
            return None;
        }
        let (ed, ml) = bytes.split_at(ED_PUBLIC_LEN);
        Some(HybridKey { ed: ed.try_into().ok()?, ml: Box::new(ml.try_into().ok()?) })
    }

    /// The key id: SHA-256 of the Ed25519 key, then the ML-DSA-65 key.
    pub fn key_id(&self) -> [u8; KEY_ID_LEN] {
        let mut h = Sha256::new();
        h.update(&self.ed);
        h.update(&self.ml[..]);
        h.finalize()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub fleet_id: [u8; 16],
    /// The fleet enrolment authority's keys.
    pub authorities: Vec<HybridKey>,
    /// Withdrawn enrolments: SHA-512 of each certificate, signatures and
    /// all, as `HOST_PROOF` carries it.
    pub revoked: Vec<[u8; 64]>,
}

impl Policy {
    pub(crate) fn authority(&self, id: &[u8; KEY_ID_LEN]) -> Option<&HybridKey> {
        self.authorities.iter().find(|a| crate::ct::eq(&a.key_id(), id))
    }
}
