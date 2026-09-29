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

//! Just enough unsigned arithmetic to check an RSA signature in a test:
//! `base^e mod n` on big-endian bytes. Slow and simple on purpose, so it is
//! not a second implementation of anything the product relies on.

pub type Limbs = Vec<u32>;

pub fn from_be(b: &[u8]) -> Limbs {
    b.rchunks(4).map(|c| c.iter().fold(0u32, |v, &x| v << 8 | u32::from(x))).collect()
}

pub fn bit(a: &Limbs, i: usize) -> bool {
    a.get(i / 32).is_some_and(|w| (w >> (i % 32)) & 1 == 1)
}

pub fn ge(a: &Limbs, n: &Limbs) -> bool {
    for i in (0..a.len().max(n.len())).rev() {
        let (x, y) = (a.get(i).copied().unwrap_or(0), n.get(i).copied().unwrap_or(0));
        if x != y {
            return x > y;
        }
    }
    true
}

pub fn sub(a: &mut Limbs, n: &Limbs) {
    let mut borrow = 0i64;
    for (i, w) in a.iter_mut().enumerate() {
        let v = i64::from(*w) - i64::from(n.get(i).copied().unwrap_or(0)) - borrow;
        borrow = i64::from(v < 0);
        *w = v.rem_euclid(1 << 32) as u32;
    }
}
