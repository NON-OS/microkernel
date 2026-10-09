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

//! The kernel's own self-attestation check, the one the bootloader makes before
//! the jump: the image's measurement at the boot epoch is enrolled under the
//! root the boot chain carries. A v4 trailer, its path folded and its STARK
//! proof of the same slot checked over words built from the image alone. The
//! root is the boot chain's, never the trailer's.
//!
//! Nothing in the kernel calls this today; it is the same check as the
//! bootloader's, kept beside the spawn gate so the two cannot drift.

use crate::security::capsule_attest::AttestError;
use nonos_attest_path::{boot_context, parse_v4, root_words, verify, Kind};
use nox_verify::attest::{words, KIND_KERNEL};
use nox_verify::statements::ATTEST;

const DEPTH: usize = 8;
const BOOT_EPOCH: u64 = 1;

/// Check the kernel's v4 self-attestation trailer against `root`. Refuses on
/// any malformed trailer, a path that does not fold, or a proof that does not
/// verify.
#[must_use = "the boot chain must halt if the kernel does not self-attest"]
pub fn verify_kernel_self_attestation(
    root: [u8; 32],
    trailer: &[u8],
    kernel_image: &[u8],
) -> Result<(), AttestError> {
    let v = parse_v4(trailer, Kind::Kernel, ATTEST.max_proof_bytes).ok_or(AttestError::Malformed)?;
    let ctx = boot_context(blake3::hash(kernel_image).as_bytes(), BOOT_EPOCH);
    if !verify(&root, DEPTH, Kind::Kernel, &ctx, v.path) {
        return Err(AttestError::Rejected);
    }
    let publics = words(root_words(&root).ok_or(AttestError::RootUnavailable)?, &ctx, KIND_KERNEL)
        .ok_or(AttestError::Rejected)?;
    nox_verify::verify(&ATTEST, v.proof, &publics).map_err(|r| AttestError::ProofRefused(r.code()))
}
