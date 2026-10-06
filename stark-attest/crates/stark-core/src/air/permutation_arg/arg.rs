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

use super::cycles::WirePermutation;
use crate::field::Fp;
use alloc::vec::Vec;

/// The single running product over the whole trace.
///
/// Disjointness proves classes do not collide. It does not prove a class is
/// enforced: a disjoint but mis telescoped product still drops a binding. That
/// is what the per binding forgeries answer, and neither check substitutes for
/// the other.
pub struct WiredPermutationArg {
    pub log_t: u32,
    pub width: usize,
    pub id: Vec<Vec<Fp>>,
    pub sigma: Vec<Vec<Fp>>,
    pub beta: Fp,
    pub gamma: Fp,
}

impl WiredPermutationArg {
    pub fn from_permutation(p: &WirePermutation, log_t: u32, beta: Fp, gamma: Fp) -> Self {
        let n = 1usize << log_t;
        let w = p.width();
        let mut id = alloc::vec![alloc::vec![Fp::ZERO; n]; w];
        let mut sigma = alloc::vec![alloc::vec![Fp::ZERO; n]; w];
        for r in 0..n {
            for (j, (idc, sgc)) in id.iter_mut().zip(sigma.iter_mut()).enumerate() {
                let k = r * w + j;
                idc[r] = Fp::from_u64(k as u64);
                let img = if r < p.rows() { p.sigma()[k] } else { k };
                sgc[r] = Fp::from_u64(img as u64);
            }
        }
        WiredPermutationArg { log_t, width: w, id, sigma, beta, gamma }
    }

    /// The running product, one column. Every cell contributes its identity in
    /// the numerator and its image in the denominator, so a cycle cancels only
    /// when every cell in it carries the same value.
    pub fn trace(&self, cells: &[Fp]) -> Vec<Fp> {
        let n = 1usize << self.log_t;
        let mut z = alloc::vec![Fp::ONE; n];
        let mut acc = Fp::ONE;
        for r in 0..n {
            z[r] = acc;
            let mut num = Fp::ONE;
            let mut den = Fp::ONE;
            for j in 0..self.width {
                let v = cells[r * self.width + j];
                num = num * (v + self.beta * self.id[j][r] + self.gamma);
                den = den * (v + self.beta * self.sigma[j][r] + self.gamma);
            }
            acc = acc * num * den.inv();
        }
        z
    }
}
