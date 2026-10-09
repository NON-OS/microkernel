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

use super::super::field::Felt;
use alloc::vec::Vec;

/// Invert every element with one field inversion: prefix products forward, one
/// inverse of the total, then walk back peeling one element off at a time.
/// The inverse of a field element is unique, so each output is the same value
/// `.inv()` would have produced; only the operation count changes. DEEP spends
/// window + 1 inversions per domain point without this, and an inversion costs
/// around sixty multiplications, and the Lagrange weights on a subgroup need
/// one per domain point.
///
/// Every input must be nonzero. DEEP's denominators are, by construction: z is
/// drawn off both the evaluation coset and the trace domain.
pub fn batch_inv<F: Felt>(vals: &[F]) -> Vec<F> {
    let mut prefix = Vec::with_capacity(vals.len());
    let mut acc = F::ONE;
    for v in vals {
        prefix.push(acc);
        acc = acc * *v;
    }
    let mut inv = acc.inv();
    let mut out = alloc::vec![F::ZERO; vals.len()];
    for i in (0..vals.len()).rev() {
        out[i] = inv * prefix[i];
        inv = inv * vals[i];
    }
    out
}
