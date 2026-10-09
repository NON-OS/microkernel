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

//! Recovering an index as a field element from the bits that already carry it.
//! A Merkle opening proves a position through its path directions, and anything
//! that hashes the same position as a scalar (a nullifier over a leaf index, say)
//! is otherwise free to hash a different one, so the two must be tied. This runs
//! `acc = sum_k bit_k * 2^k`, one row per bit: the bits ride the trace, where the
//! assembly binds them to the opening directions that already carry the position,
//! the powers of two are periodic, and the accumulator starts at a boundary of
//! zero. The final accumulator is witness, bound to wherever the scalar is
//! consumed. Same shape as `IndexPoint`, with a sum where that has a product.

use super::super::field::{Felt, Fp, Fp2};
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

#[derive(Clone)]
pub struct IndexScalar {
    log_len: u32,
    bits: usize,
    index: u64,
}

impl IndexScalar {
    /// The chain recovering `index` over `bits` bits, low bit first.
    pub fn new(bits: usize, index: u64) -> IndexScalar {
        let log_len = (bits + 1).next_power_of_two().trailing_zeros();
        IndexScalar { log_len, bits, index }
    }

    /// The row carrying bit `k`, for the binding to an opening direction.
    pub fn bit_row(&self, k: usize) -> usize {
        k
    }

    /// The row the recovered index sits on, after the last bit.
    pub fn value_row(&self) -> usize {
        self.bits
    }

    /// The accumulator column, and the bit column beside it.
    pub const ACC: usize = 0;
    pub const BIT: usize = 1;

    /// The witness: the running sum and the bit consumed on each row.
    pub fn trace(&self) -> Vec<Fp> {
        let n = 1usize << self.log_len;
        let mut trace = alloc::vec![Fp::ZERO; n * 2];
        let mut acc = 0u64;
        for r in 0..n {
            let bit = if r < self.bits { (self.index >> r) & 1 } else { 0 };
            trace[r * 2] = Fp::from_u64(acc);
            trace[r * 2 + 1] = Fp::from_u64(bit);
            if r < self.bits {
                acc += bit << r;
            }
        }
        trace
    }

    /// The transition over any field, for in-circuit recomputation.
    pub fn transition_gen<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        self.transition_impl(window, periodic)
    }

    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let (acc, bit) = (window[0], window[1]);
        let acc_next = window[2];
        let pow = periodic[0];
        let one = F::ONE;
        alloc::vec![
            // The accumulator takes the bit at its weight, on the rows that carry
            // one; past the last bit the selector is zero and the sum is carried.
            acc_next - (acc + bit * pow),
            // A witnessed bit must be a bit, or one cell could stand for any index.
            bit * (one - bit),
        ]
    }
}

impl Air for IndexScalar {
    fn log_trace_len(&self) -> u32 {
        self.log_len
    }

    fn trace_width(&self) -> usize {
        2
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        2
    }

    fn num_transition(&self) -> usize {
        2
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let n = 1usize << self.log_len;
        // Weight of the bit on each row, zero once the bits run out, so the rows
        // past the index carry the accumulator unchanged.
        let mut pow = alloc::vec![Fp::ZERO; n];
        for (r, slot) in pow.iter_mut().enumerate().take(self.bits) {
            *slot = Fp::from_u64(1u64 << r);
        }
        alloc::vec![pow]
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // The sum starts empty. Without this a prover offsets every index by a
        // constant and the recovered scalar means nothing.
        alloc::vec![(Self::ACC, 0, Fp::ZERO)]
    }
}

impl AirExt for IndexScalar {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}
