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

//! A, the root of a declared syscall set.
//!
//! The set is the LinuxAppPolicy's `syscall_set`: 65 bytes, bit b of the set
//! being bit b % 8 of byte b / 8, set when bucket b may succeed. It comes from
//! the signed listing, never from the kernel.

use crate::hist::BUCKETS;
use crate::poseidon::chain;

pub const SET_BYTES: usize = 65;

/// "NONOSPOL", lane 0 of the set's chain.
pub const POL_DOMAIN: u64 = 0x4E4F_4E4F_5350_4F4C;

/// 513 bit words padded to whole blocks of four.
const SET_WORDS: usize = 516;

/// Whether the set lets bucket `b` succeed. Nothing past 512 is in a set.
pub fn allows(set: &[u8; SET_BYTES], b: usize) -> bool {
    b < BUCKETS && set[b / 8] >> (b % 8) & 1 == 1
}

/// `None` for a set with a bit past 512: there is no such bucket, and a root
/// that ignored the bit would let two sets share one A.
pub fn syscall_root(set: &[u8; SET_BYTES]) -> Option<[u64; 4]> {
    if set[SET_BYTES - 1] & !1 != 0 {
        return None;
    }
    let mut w = [0u64; SET_WORDS];
    for (b, word) in w.iter_mut().enumerate().take(BUCKETS) {
        *word = u64::from(allows(set, b));
    }
    chain(POL_DOMAIN, &w)
}
