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

//! The transparent, post-quantum spawn gate. A capsule ships a STARK proof that
//! its own measurement is a leaf under the kernel policy root, bound to the
//! capsule context. The gate measures the ELF itself and pins that measurement
//! as the opened leaf, so a proof about any other enrolled capsule is refused.
//! The trusted root is the kernel's own, never the trailer's.

use super::error::AttestError;
use super::layout::{POLICY_EPOCH, POLICY_TREE_DEPTH};
use crate::crypto::stark::air::{verify_public_trailer_digest, PUBLIC_TRAILER_MAGIC};

pub(super) const MAGIC: &[u8; 8] = PUBLIC_TRAILER_MAGIC;

/// Verify a capsule's attestation against `policy`, bound to its measurement
/// `digest` (see `measure`), its granted capabilities and the epoch.
///
/// The root is a parameter rather than a lookup, so a capsule built on this
/// machine clears exactly the bar a shipped one does. Only whose tree it is
/// proved against differs.
#[must_use = "a capsule must not be spawned unless its attestation verifies"]
pub(super) fn verify_against(
    trailer: &[u8],
    digest: &[u8; 32],
    granted_caps: u64,
    policy: &[u8; 32],
) -> Result<[u8; 32], AttestError> {
    if !trailer.starts_with(MAGIC) {
        return Err(AttestError::Malformed);
    }
    let capsule_hash = *digest;
    let mut ctx = [0u8; 48];
    ctx[..32].copy_from_slice(&capsule_hash);
    ctx[32..40].copy_from_slice(&granted_caps.to_be_bytes());
    ctx[40..48].copy_from_slice(&POLICY_EPOCH.to_be_bytes());
    if verify_public_trailer_digest(policy, POLICY_TREE_DEPTH, digest, trailer, &ctx) {
        Ok(capsule_hash)
    } else {
        Err(AttestError::Rejected)
    }
}
