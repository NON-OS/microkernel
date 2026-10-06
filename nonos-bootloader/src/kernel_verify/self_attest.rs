// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Kernel self-attestation, checked before the jump. A v4 trailer: the path is
//! folded from this kernel's leaf to the enrolled root, then the STARK proof of
//! the same slot is checked over words built from the kernel bytes alone.

use nonos_attest_path::{boot_context, parse_v4, root_words, verify, Kind};
use nox_verify::attest::{words, KIND_KERNEL};
use nox_verify::statements::ATTEST;

pub(crate) const DEPTH: usize = 8;
pub(crate) const BOOT_EPOCH: u64 = 1;

/*
 * The enrolled kernel measurement root the boot chain trusts, provisioned by
 * `build.rs` from `NONOS_KERNEL_ATTEST_ROOT`. Only a dev build may carry it
 * zeroed, which accepts nothing. A static read through black_box, so it stays
 * 32 contiguous bytes in the image where the build receipt finds it.
 */
include!(concat!(env!("OUT_DIR"), "/kernel_attest_root.rs"));

/// The root the gate folds to, for the handoff.
pub(crate) fn enrolled_root() -> [u8; 32] {
    *core::hint::black_box(&KERNEL_ATTEST_ROOT)
}

/// True only when the path folds from exactly this kernel's leaf to exactly
/// this root and the STARK proof of that slot verifies.
#[must_use = "the boot chain must halt if the kernel does not self-attest"]
pub fn verify_kernel_self_attestation(kernel_bytes: &[u8], trailer: &[u8]) -> bool {
    let root = enrolled_root();
    if root == [0u8; 32] {
        return false;
    }
    let Some(v) = parse_v4(trailer, Kind::Kernel, ATTEST.max_proof_bytes) else {
        return false;
    };
    let ctx = boot_context(blake3::hash(kernel_bytes).as_bytes(), BOOT_EPOCH);
    if !verify(&root, DEPTH, Kind::Kernel, &ctx, v.path) {
        return false;
    }
    let publics = root_words(&root).and_then(|r| words(r, &ctx, KIND_KERNEL));
    publics.is_some_and(|w| nox_verify::verify(&ATTEST, v.proof, &w).is_ok())
}

/*
 * A development kernel's trailer is the path alone. The context is built from
 * this kernel's bytes as above and the path must fold to the same enrolled
 * root; only the STARK proof is absent. Built only with dev-attest, which only
 * the development loader policy turns on.
 */
#[cfg(feature = "dev-attest")]
pub(crate) fn verify_kernel_path_only(kernel_bytes: &[u8], trailer: &[u8]) -> bool {
    let root = enrolled_root();
    if root == [0u8; 32] {
        return false;
    }
    let ctx = boot_context(blake3::hash(kernel_bytes).as_bytes(), BOOT_EPOCH);
    verify(&root, DEPTH, Kind::Kernel, &ctx, trailer)
}

/// The STARK proof's length in a v4 kernel trailer, zero for any other bytes.
pub(crate) fn proof_len(trailer: &[u8]) -> usize {
    parse_v4(trailer, Kind::Kernel, ATTEST.max_proof_bytes).map_or(0, |v| v.proof.len())
}
