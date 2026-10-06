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

//! A 1 KiB memory block and the compression function G of RFC 9106,
//! section 3.5, with the permutation P of section 3.6.

use super::wipe::wipe;

/// A block as 128 little-endian words.
pub(super) type Block = [u64; 128];

/// GB: BLAKE2b's G with each addition carrying the product of the low halves.
fn gb(v: &mut Block, a: usize, b: usize, c: usize, d: usize) {
    let fma = |x: u64, y: u64| {
        let lo = (x & 0xFFFF_FFFF).wrapping_mul(y & 0xFFFF_FFFF);
        x.wrapping_add(y).wrapping_add(lo.wrapping_mul(2))
    };
    v[a] = fma(v[a], v[b]);
    v[d] = (v[d] ^ v[a]).rotate_right(32);
    v[c] = fma(v[c], v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(24);
    v[a] = fma(v[a], v[b]);
    v[d] = (v[d] ^ v[a]).rotate_right(16);
    v[c] = fma(v[c], v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(63);
}

/// P over the sixteen words at `at`, which name eight 16-byte registers.
fn permute(v: &mut Block, at: [usize; 16]) {
    gb(v, at[0], at[4], at[8], at[12]);
    gb(v, at[1], at[5], at[9], at[13]);
    gb(v, at[2], at[6], at[10], at[14]);
    gb(v, at[3], at[7], at[11], at[15]);
    gb(v, at[0], at[5], at[10], at[15]);
    gb(v, at[1], at[6], at[11], at[12]);
    gb(v, at[2], at[7], at[8], at[13]);
    gb(v, at[3], at[4], at[9], at[14]);
}

/// G(x, y) written to `out`: over R = x ^ y, P on each row of eight
/// registers, then on each column, and the result XORed with R. With
/// `keep`, the result is XORed into what `out` already holds, as passes
/// after the first do.
pub(super) fn compress(x: &Block, y: &Block, out: &mut Block, keep: bool) {
    let mut r: Block = core::array::from_fn(|i| x[i] ^ y[i]);
    let mut q = r;
    for row in 0..8 {
        permute(&mut q, core::array::from_fn(|i| row * 16 + i));
    }
    for col in 0..8 {
        permute(&mut q, core::array::from_fn(|i| 2 * col + (i / 2) * 16 + (i % 2)));
    }
    for i in 0..128 {
        let z = q[i] ^ r[i];
        out[i] = if keep { out[i] ^ z } else { z };
    }
    wipe(&mut q);
    wipe(&mut r);
}
