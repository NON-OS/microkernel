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

//! Which futex waiters a wake takes. Pure, so the host proofs hold it.

use alloc::vec::Vec;

/// FUTEX_BITSET_MATCH_ANY: the bitset a plain WAIT and WAKE carry.
pub const MATCH_ANY: u32 = u32::MAX;

/// The places in `waits` (thread, word) of the waiters a wake of `uaddr`
/// for `bits` takes, oldest first and at most `count`: those parked on that
/// word whose own bitset, `bits_of`, shares a bit with the wake's. One that
/// shares none is passed over and stays parked, as Linux leaves it, so a
/// wake meant for another waiter can never use up the one it was meant for.
pub fn pick(
    waits: &[(u32, u64)],
    bits_of: impl Fn(u32) -> u32,
    uaddr: u64,
    bits: u32,
    count: u64,
) -> Vec<usize> {
    let wanted =
        |&(_, &(tid, word)): &(usize, &(u32, u64))| word == uaddr && bits_of(tid) & bits != 0;
    let mut out = Vec::new();
    for (at, _) in waits.iter().enumerate().filter(wanted) {
        if out.len() as u64 >= count {
            break;
        }
        out.push(at);
    }
    out
}
