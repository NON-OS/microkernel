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


//! An Equi-X solution: eight 16-bit indices whose eight hashes sum to zero
//! in the low 60 bits, with partial sums that vanish in 15 and 30 bits.
//!
//! The indices are kept in one canonical order, a binary tree where each left
//! branch is not greater than its right, so a solution has exactly one valid
//! encoding and cannot be replayed under a permutation.

use crate::hashx::HashX;

pub const NUM_IDX: usize = 8;
pub const SOLUTION_BYTES: usize = 16;
pub const STAGE1_MASK: u64 = (1 << 15) - 1;
pub const STAGE2_MASK: u64 = (1 << 30) - 1;
pub const FULL_MASK: u64 = (1 << 60) - 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Solution {
    pub idx: [u16; NUM_IDX],
}

/// Why a solution does not verify.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VerifyError {
    /// The challenge selects a program that fails the uniformity rules, so
    /// nothing can be a solution to it.
    Challenge,
    /// The indices are not in their canonical order.
    Order,
    /// A pair or a quad does not sum to zero in its bits.
    PartialSum,
    /// The eight hashes do not sum to zero in the low 60 bits.
    FinalSum,
}

pub fn tree_idx2(idx: &[u16]) -> u32 {
    u32::from(idx[1]) << 16 | u32::from(idx[0])
}

pub fn tree_idx4(idx: &[u16]) -> u64 {
    u64::from(idx[3]) << 48 | u64::from(idx[2]) << 32 | u64::from(idx[1]) << 16 | u64::from(idx[0])
}

impl Solution {
    /// The wire form: each index little endian, in order.
    pub fn to_bytes(&self) -> [u8; SOLUTION_BYTES] {
        let mut out = [0u8; SOLUTION_BYTES];
        for (pair, idx) in out.chunks_exact_mut(2).zip(self.idx.iter()) {
            pair.copy_from_slice(&idx.to_le_bytes());
        }
        out
    }

    pub fn from_bytes(bytes: &[u8; SOLUTION_BYTES]) -> Self {
        let mut idx = [0u16; NUM_IDX];
        for (i, pair) in idx.iter_mut().zip(bytes.chunks_exact(2)) {
            *i = u16::from_le_bytes([pair[0], pair[1]]);
        }
        Self { idx }
    }

    pub fn in_order(&self) -> bool {
        let x = &self.idx;
        tree_idx4(&x[0..4]) <= tree_idx4(&x[4..8])
            && tree_idx2(&x[0..2]) <= tree_idx2(&x[2..4])
            && tree_idx2(&x[4..6]) <= tree_idx2(&x[6..8])
            && x[0] <= x[1]
            && x[2] <= x[3]
            && x[4] <= x[5]
            && x[6] <= x[7]
    }
}

/// Checks `solution` against `challenge`. Order is checked first and costs no
/// hashing, so a malformed solution is refused before any program is built.
pub fn verify(challenge: &[u8], solution: &Solution) -> Result<(), VerifyError> {
    if !solution.in_order() {
        return Err(VerifyError::Order);
    }
    let hashx = HashX::new(challenge).ok_or(VerifyError::Challenge)?;
    verify_with(&hashx, solution)
}

pub fn verify_with(hashx: &HashX, solution: &Solution) -> Result<(), VerifyError> {
    let x = &solution.idx;
    let pair = |a: u16, b: u16| hashx.hash(u64::from(a)).wrapping_add(hashx.hash(u64::from(b)));
    let p0 = pair(x[0], x[1]);
    if p0 & STAGE1_MASK != 0 {
        return Err(VerifyError::PartialSum);
    }
    let p1 = pair(x[2], x[3]);
    if p1 & STAGE1_MASK != 0 {
        return Err(VerifyError::PartialSum);
    }
    let p4 = p0.wrapping_add(p1);
    if p4 & STAGE2_MASK != 0 {
        return Err(VerifyError::PartialSum);
    }
    let p2 = pair(x[4], x[5]);
    if p2 & STAGE1_MASK != 0 {
        return Err(VerifyError::PartialSum);
    }
    let p3 = pair(x[6], x[7]);
    if p3 & STAGE1_MASK != 0 {
        return Err(VerifyError::PartialSum);
    }
    let p5 = p2.wrapping_add(p3);
    if p5 & STAGE2_MASK != 0 {
        return Err(VerifyError::PartialSum);
    }
    if p4.wrapping_add(p5) & FULL_MASK != 0 {
        return Err(VerifyError::FinalSum);
    }
    Ok(())
}
