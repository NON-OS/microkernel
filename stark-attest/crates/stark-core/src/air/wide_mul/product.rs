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

use super::limbs::{split, LIMB_BITS, LIMB_MASK, N_LIMBS};

pub const N_OUT: usize = 2 * N_LIMBS;

/// The full product as eight 16 bit limbs, with the carry at each weight.
pub struct Product {
    pub out: [u64; N_OUT],
    pub carry: [u64; N_OUT],
}

/// Schoolbook over limbs. Each weight sums the partial products that land on it
/// plus the incoming carry; both stay far below the field, so the relation holds
/// over the integers rather than modulo p.
pub fn wide_mul(a: u64, b: u64) -> Product {
    let (al, bl) = (split(a), split(b));
    let mut out = [0u64; N_OUT];
    let mut carry = [0u64; N_OUT];
    let mut c = 0u64;
    for k in 0..N_OUT {
        let mut s = c;
        for i in 0..N_LIMBS {
            let j = k as i64 - i as i64;
            if j >= 0 && (j as usize) < N_LIMBS {
                s += al[i] * bl[j as usize];
            }
        }
        carry[k] = c;
        out[k] = s & LIMB_MASK;
        c = s >> LIMB_BITS;
    }
    Product { out, carry }
}
