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

//! One hop's keystreams, digests and windows.

use crate::crypto::sha1::Sha1;
use crate::crypto::Ctr128Be;
use crate::ntor::KEY_MATERIAL_BYTES;

/// One hop's crypto: a keystream and a running digest in each direction.
pub struct Hop {
    pub forward: Ctr128Be,
    pub backward: Ctr128Be,
    pub forward_digest: Sha1,
    pub backward_digest: Sha1,
    /// The digest of the last cell that arrived and was ours, kept because a
    /// SENDME has to name it.
    pub last_seen: [u8; 20],
    /// Cells we may still send before the far end has to grant more.
    pub package_window: i32,
    /// Cells that may still arrive before we owe a SENDME.
    pub deliver_window: i32,
}

impl Hop {
    /*
     * The key material is `Df || Db || Kf || Kb`: twenty bytes that seed the
     * forward digest, twenty that seed the backward one, then a sixteen byte
     * key each way. The digests are seeded by being fed their twenty bytes,
     * not by having their state overwritten, which is why this is an
     * `update` and not an assignment.
     */
    pub fn new(keys: &[u8; KEY_MATERIAL_BYTES]) -> Self {
        let mut forward_digest = Sha1::new();
        forward_digest.update(&keys[..20]);
        let mut backward_digest = Sha1::new();
        backward_digest.update(&keys[20..40]);
        let mut forward_key = [0u8; 16];
        forward_key.copy_from_slice(&keys[40..56]);
        let mut backward_key = [0u8; 16];
        backward_key.copy_from_slice(&keys[56..72]);
        Self {
            forward: Ctr128Be::new(&forward_key),
            backward: Ctr128Be::new(&backward_key),
            forward_digest,
            backward_digest,
            last_seen: [0u8; 20],
            package_window: super::window::CIRCUIT_START,
            deliver_window: super::window::CIRCUIT_START,
        }
    }
}
