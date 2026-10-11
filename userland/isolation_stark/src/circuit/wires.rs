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

//! The copy constraints. Each gate's live bit is the bit A's chain absorbs
//! for its bucket, and its value's low limb the success count H's chain
//! absorbs; the chains' roots are the public words. A gate cell no chain
//! committed, or a public word no root carries, is one the prover could
//! choose.

use alloc::vec::Vec;

use nonos_proof_abi::BUCKETS as N_BUCKETS;
use nonos_stark::air::LiveGate;
use stark_proofs::shield::wire_class::{tie, Class};

use super::layout::{root, word, A, A_BLOCKS, GATES, H, H_BLOCKS, PUBLICS};

pub(super) fn classes(off: &[usize]) -> Vec<Class> {
    let at = |region: usize, (row, col): (usize, usize)| (off[region] + row, col);
    let gate = |b: usize, col: usize| (off[GATES + b], col);
    let public = |k: usize| (off[PUBLICS] + k, 0);
    let mut out = Vec::with_capacity(2 * N_BUCKETS + 8);
    for b in 0..N_BUCKETS {
        out.push(tie(&[gate(b, LiveGate::LIVE), at(A, word(b))]));
        out.push(tie(&[gate(b, LiveGate::value_col(0)), at(H, word(2 * b))]));
    }
    for lane in 0..4 {
        out.push(tie(&[public(lane), at(H, root(H_BLOCKS, lane))]));
        out.push(tie(&[public(4 + lane), at(A, root(A_BLOCKS, lane))]));
    }
    out
}
