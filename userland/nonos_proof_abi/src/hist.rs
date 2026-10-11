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

//! The family histogram and its commitment Hc.
//!
//! The kernel counts every answered call in one of 513 buckets by number and
//! one of two sides by the sign of its return value. That is a numeric
//! convention and nothing more: the kernel learns no meaning from it.

use alloc::vec::Vec;

use crate::poseidon::chain;

pub const BUCKETS: usize = 513;
pub const SIDES: usize = 2;
pub const HIST_WORDS: usize = BUCKETS * SIDES;

/// "NONOSISO", lane 0 of the histogram chain.
pub const ISO_DOMAIN: u64 = 0x4E4F_4E4F_5349_534F;

/// Numbers below 512 have their own bucket; every larger one shares the last.
pub fn bucket(nr: u64) -> usize {
    if nr < 512 {
        nr as usize
    } else {
        BUCKETS - 1
    }
}

/// Side 0 for a return value that is non-negative as an i64, side 1 otherwise.
pub fn side(ret: u64) -> usize {
    usize::from((ret as i64) < 0)
}

/// Hc over `h`, flattened bucket-major and side-minor, two zero words, then
/// the salt. `None` for a salt word not below p.
pub fn hist_commit(h: &[u32; HIST_WORDS], salt: &[u64; 4]) -> Option<[u64; 4]> {
    let mut w = Vec::with_capacity(HIST_WORDS + 6);
    w.extend(h.iter().map(|&c| u64::from(c)));
    w.extend_from_slice(&[0, 0]);
    w.extend_from_slice(salt);
    chain(ISO_DOMAIN, &w)
}
