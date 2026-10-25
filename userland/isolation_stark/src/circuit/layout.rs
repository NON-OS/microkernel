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

//! Where things sit. The regions stack as H's chain, A's chain, one gate per
//! bucket and the publics; a chain absorbs block m in its state's lanes 4 to 7 on
//! row 0 when m is 0, and in its sibling columns on row 32 m - 1 otherwise,
//! and holds its root in lanes 0 to 3 of row 32 m once m blocks are in.

use alloc::vec::Vec;

use nonos_proof_abi::{BUCKETS, HIST_WORDS};

pub(super) const H: usize = 0;
pub(super) const A: usize = 1;
/// The first of the 513 gates, one region per bucket.
pub(super) const GATES: usize = 2;
pub(super) const PUBLICS: usize = GATES + BUCKETS;

/// H's 1026 words and two zeros, then the salt: 258 blocks.
pub(super) const H_BLOCKS: usize = HIST_WORDS / 4 + 2;
/// 513 bits and three zeros: 129 blocks.
pub(super) const A_BLOCKS: usize = 129;

const ROUNDS: usize = 32;
/// Width of a Poseidon state, the sibling columns follow a direction column.
const STATE: usize = 8;
const SIBLING: usize = STATE + 1;

/// The (row, column) inside a chain region of the word it absorbs at `j`.
pub(super) fn word(j: usize) -> (usize, usize) {
    let (m, lane) = (j / 4, j % 4);
    if m == 0 {
        (0, 4 + lane)
    } else {
        (m * ROUNDS - 1, SIBLING + lane)
    }
}

/// The (row, column) of a chain's root lane once `blocks` are absorbed.
pub(super) fn root(blocks: usize, lane: usize) -> (usize, usize) {
    (blocks * ROUNDS, lane)
}

/// Each region's kind. The chains differ in length, so each is its own; the
/// gates share one, and so one selector.
pub(super) fn kinds() -> Vec<usize> {
    (0..=PUBLICS)
        .map(|r| match r {
            H | A => r,
            PUBLICS => 3,
            _ => 2,
        })
        .collect()
}
