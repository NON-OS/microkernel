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

use super::bits::Bools;
use super::tables::{prob_index, BANDS, CATS, ZIGZAG};

/// Tokens of one 4x4 block (RFC 6386 13) of type `t`, from position
/// `first` with neighbour context `ctx`, dequantized by `dq` (DC step,
/// AC step) into `out` in raster order. Returns one past the last
/// nonzero position, 0 for an empty block.
pub(super) fn block_tokens(
    br: &mut Bools,
    probs: &[u8],
    (t, ctx): (usize, usize),
    dq: [i32; 2],
    first: usize,
    out: &mut [i16],
) -> usize {
    let at = |n: usize, c: usize| prob_index(t, BANDS[n], c);
    let mut n = first;
    let mut p = at(n, ctx);
    while n < 16 {
        if !br.bit(probs[p]) {
            return n;
        }
        while !br.bit(probs[p + 1]) {
            n += 1;
            if n == 16 {
                return 16;
            }
            p = at(n, 0);
        }
        let (v, next) =
            if !br.bit(probs[p + 2]) { (1, 1) } else { (large(br, &probs[p..p + 11]), 2) };
        let v = if br.bit(128) { -v } else { v };
        let q = v * dq[usize::from(n > 0)];
        out[ZIGZAG[n]] = q.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        n += 1;
        p = at(n, next);
    }
    16
}

/// A token of magnitude two or more: DCT tokens 2..4 or a category with
/// extra bits (RFC 6386 13.2).
fn large(br: &mut Bools, p: &[u8]) -> i32 {
    if !br.bit(p[3]) {
        return if !br.bit(p[4]) { 2 } else { 3 + br.bit(p[5]) as i32 };
    }
    if !br.bit(p[6]) {
        if !br.bit(p[7]) {
            return 5 + br.bit(159) as i32;
        }
        let v = 7 + 2 * br.bit(165) as i32;
        return v + br.bit(145) as i32;
    }
    let b1 = br.bit(p[8]) as usize;
    let b0 = br.bit(p[9 + b1]) as usize;
    let cat = 2 * b1 + b0;
    let extra = CATS[cat].iter().fold(0i32, |v, &pr| 2 * v + br.bit(pr) as i32);
    extra + 3 + (8 << cat)
}
