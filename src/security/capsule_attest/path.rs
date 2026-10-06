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

//! The spawn gate. A capsule ships a v4 trailer: the path from its leaf to the
//! kernel policy root, and a STARK proof of the same slot. The gate computes the
//! leaf's context itself from the ELF it is about to run, the capabilities it is
//! about to grant and the policy epoch, folds the path, then checks the proof
//! over the statement's words built from that same context. Both must hold, and
//! the trusted root is always the kernel's own, never the trailer's.

use super::error::AttestError;
use super::layout::{POLICY_EPOCH, POLICY_TREE_DEPTH};
use nonos_attest_path::{capsule_context, parse_v4, root_words, verify, Kind};
use nox_verify::attest::{words, KIND_CAPSULE};
use nox_verify::statements::ATTEST;

/// Check a capsule's v4 trailer against `policy`, for its measurement (the
/// BLAKE3 `digest` the gate took once, in serve units), its granted
/// capabilities and the epoch. The root is a parameter, so a capsule
/// built on this machine clears exactly the bar a shipped one does.
#[must_use = "a capsule must not be spawned unless its attestation verifies"]
pub(super) fn verify_against(
    trailer: &[u8],
    digest: &[u8; 32],
    granted_caps: u64,
    policy: &[u8; 32],
) -> Result<[u8; 32], AttestError> {
    #[cfg(feature = "nonos-dev-attest")]
    if trailer.starts_with(&nonos_attest_path::MAGIC) {
        return dev_path(trailer, digest, granted_caps, policy);
    }
    let v = parse_v4(trailer, Kind::Capsule, ATTEST.max_proof_bytes).ok_or(AttestError::Malformed)?;
    let ctx = capsule_context(digest, granted_caps, POLICY_EPOCH);
    if !verify(policy, POLICY_TREE_DEPTH, Kind::Capsule, &ctx, v.path) {
        return Err(AttestError::Rejected);
    }
    let root = root_words(policy).ok_or(AttestError::RootUnavailable)?;
    let publics = words(root, &ctx, KIND_CAPSULE).ok_or(AttestError::Rejected)?;
    nox_verify::verify(&ATTEST, v.proof, &publics).map_err(|r| AttestError::ProofRefused(r.code()))?;
    Ok(*digest)
}

/*
 * A development image's capsules carry the path alone, with no STARK proof, so
 * a test image is enrolled in seconds instead of hours. Everything else is the
 * check above: the context is built here from the ELF about to run, the
 * capabilities about to be granted and the epoch, and the path must fold to
 * the kernel's own root. A capsule whose bytes, capabilities or slot differ is
 * refused exactly as a release refuses it. The feature exists only in images
 * built with the loader's development policy, and never with
 * nonos-production (src/lib.rs).
 */
#[cfg(feature = "nonos-dev-attest")]
fn dev_path(
    trailer: &[u8],
    digest: &[u8; 32],
    granted_caps: u64,
    policy: &[u8; 32],
) -> Result<[u8; 32], AttestError> {
    use core::sync::atomic::{AtomicBool, Ordering};
    static SAID: AtomicBool = AtomicBool::new(false);
    if !SAID.swap(true, Ordering::Relaxed) {
        crate::sys::serial::print(
            b"[ZK-ATTEST] development image: paths only, no STARK proofs, never a release\n",
        );
    }
    let ctx = capsule_context(digest, granted_caps, POLICY_EPOCH);
    if verify(policy, POLICY_TREE_DEPTH, Kind::Capsule, &ctx, trailer) {
        Ok(*digest)
    } else {
        Err(AttestError::Rejected)
    }
}
