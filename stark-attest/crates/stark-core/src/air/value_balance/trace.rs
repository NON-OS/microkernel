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

use super::super::super::field::Fp;
use super::air::{ValueBalance, LIMB_SHIFT};
use super::leg::Leg;
use alloc::vec;
use alloc::vec::Vec;

impl ValueBalance {
    /// Terms are supplied in the same order as `legs`, so the trace and the sign
    /// column cannot drift apart.
    pub fn trace(&self, terms: &[(Fp, Fp)]) -> Vec<Fp> {
        let n = 1usize << self.log_t;
        let mut t = vec![Fp::ZERO; n * 4];
        let mut acc = Fp::ZERO;
        for r in 0..n {
            let (lo, hi) = terms.get(r).copied().unwrap_or((Fp::ZERO, Fp::ZERO));
            let value = lo + hi * Fp::from_u64(LIMB_SHIFT);
            t[r * 4] = acc;
            t[r * 4 + 1] = lo;
            t[r * 4 + 2] = hi;
            t[r * 4 + 3] = value;
            acc = acc + self.legs.get(r).copied().unwrap_or(Leg::Pad).sign() * value;
        }
        t
    }
}
