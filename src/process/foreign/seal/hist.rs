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

//! H, the family's histogram, and Hc, its commitment.
//!
//! A call counts in one of 513 buckets by its number, on side 1 when it was
//! refused and side 0 otherwise. Numbers only: the kernel does not know what
//! any of them means.

use alloc::vec::Vec;

pub(crate) const BUCKETS: usize = 513;
pub(crate) const HIST_WORDS: usize = 2 * BUCKETS;

/// "NONOSISO", lane 0 of the chain.
const ISO_DOMAIN: u64 = 0x4E4F_4E4F_5349_534F;

/// The longest path nox_verify folds in one call.
const FOLD_MAX: usize = 32;

/// EPERM and ENOSYS as a guest reads them. Only these two are refusals;
/// every other answer counts as the call going through.
const EPERM: u64 = -1i64 as u64;
const ENOSYS: u64 = -38i64 as u64;

pub(crate) fn refusal(ret: u64) -> bool {
    ret == EPERM || ret == ENOSYS
}

/// Where a call lands: bucket-major, side 1 for a refusal.
pub(crate) fn slot(nr: u64, refused: bool) -> usize {
    let bucket = if nr < 512 { nr as usize } else { BUCKETS - 1 };
    2 * bucket + usize::from(refused)
}

/*
 * Hc is a chain: node = compress(node, block), from [ISO_DOMAIN, 0, 0, 0],
 * over H four words at a time (the last block's two missing words zero),
 * then the salt. That is nox_verify's path fold with every sibling on the
 * right of the node, so the kernel uses the gates' own Poseidon for it.
 */
pub(crate) fn hist_commit(h: &[u32; HIST_WORDS], salt: &[u64; 4]) -> Option<[u64; 4]> {
    let mut blocks: Vec<[u64; 4]> = h
        .chunks(4)
        .map(|c| core::array::from_fn(|i| c.get(i).map_or(0, |&v| u64::from(v))))
        .collect();
    blocks.push(*salt);
    let left = [false; FOLD_MAX];
    let mut node = [ISO_DOMAIN, 0, 0, 0];
    for part in blocks.chunks(FOLD_MAX) {
        node = nox_verify::attest::fold_path(node, part, &left[..part.len()])?;
    }
    Some(node)
}
