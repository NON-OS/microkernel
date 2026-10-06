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

//! `base^e mod n` by square and multiply, for the test's RSA check.

use crate::bignum::{bit, from_be, ge, sub, Limbs};

/// (a * b) mod n, by shifting the product in one bit at a time.
fn mulmod(a: &Limbs, b: &Limbs, n: &Limbs) -> Limbs {
    let mut r: Limbs = vec![0; n.len() + 1];
    for i in (0..a.len() * 32).rev() {
        let mut carry = 0u32;
        for w in r.iter_mut() {
            let next = *w >> 31;
            *w = *w << 1 | carry;
            carry = next;
        }
        if ge(&r, n) {
            sub(&mut r, n);
        }
        if bit(a, i) {
            let mut c = 0u64;
            for (j, w) in r.iter_mut().enumerate() {
                let v = u64::from(*w) + u64::from(b.get(j).copied().unwrap_or(0)) + c;
                *w = v as u32;
                c = v >> 32;
            }
            while ge(&r, n) {
                sub(&mut r, n);
            }
        }
    }
    r
}

pub fn modpow(base: &[u8], e: &[u8], n: &[u8]) -> Vec<u8> {
    let (n, e, b) = (from_be(n), from_be(e), from_be(base));
    let mut r: Limbs = vec![1];
    for i in (0..e.len() * 32).rev() {
        r = mulmod(&r, &r, &n);
        if bit(&e, i) {
            r = mulmod(&r, &b, &n);
        }
    }
    r.iter().rev().flat_map(|w| w.to_be_bytes()).collect()
}
