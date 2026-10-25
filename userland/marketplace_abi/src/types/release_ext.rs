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

//! What schema 3 adds to a release. All of it sits under the publisher's
//! release signature (domain `NONOS.marketplace.release.v3`).

extern crate alloc;

use alloc::vec::Vec;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReleaseExt {
    /// BLAKE3 of the commerce domain and the entry's commerce block bytes.
    pub commerce_hash: [u8; 32],
    /// A LinuxAppPolicy (plan/30): exactly 256 bytes, or empty.
    pub guest_policy: Vec<u8>,
    /// Root of the 1024-byte chunk tree (plan/21); zero unless escrowed.
    pub chunk_root: [u8; 32],
    /// The package's length in bytes; zero unless escrowed.
    pub content_len: u64,
    /// Anchors of the publisher's own reference proofs (plan/11).
    pub proof_anchors: Vec<[u8; 32]>,
    /// The payee's ReleaseAttest (plan/23) over this release, r, s, v; zero
    /// when the entry has no payee. Two of them differing are slashable.
    pub release_attest_sig: [u8; 65],
}
