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

use nonos_attest_path::{boot_context, parse_v4, root_words, verify, Kind};
use nox_verify::attest::{words, KIND_KERNEL};
use nox_verify::statements::ATTEST;

pub(crate) const DEPTH: usize = 8;
pub(crate) const BOOT_EPOCH: u64 = 1;

include!(concat!(env!("OUT_DIR"), "/kernel_attest_root.rs"));

pub(crate) fn enrolled_root() -> [u8; 32] {
    *core::hint::black_box(&KERNEL_ATTEST_ROOT)
}

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

#[cfg(feature = "dev-attest")]
pub(crate) fn verify_kernel_path_only(kernel_bytes: &[u8], trailer: &[u8]) -> bool {
    let root = enrolled_root();
    if root == [0u8; 32] {
        return false;
    }
    let ctx = boot_context(blake3::hash(kernel_bytes).as_bytes(), BOOT_EPOCH);
    verify(&root, DEPTH, Kind::Kernel, &ctx, trailer)
}

pub(crate) fn proof_len(trailer: &[u8]) -> usize {
    parse_v4(trailer, Kind::Kernel, ATTEST.max_proof_bytes).map_or(0, |v| v.proof.len())
}
