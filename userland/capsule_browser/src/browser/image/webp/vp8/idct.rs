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

/* sqrt(2) cos(pi/8) - 1 and sqrt(2) sin(pi/8) in 16-bit fixed point; the
 * product is taken wide so hostile coefficients cannot overflow it. */
fn mul1(a: i32) -> i32 {
    ((a as i64 * 20091) >> 16) as i32 + a
}

fn mul2(a: i32) -> i32 {
    ((a as i64 * 35468) >> 16) as i32
}

/// Inverse transform of a 4x4 block (RFC 6386 14.3), added to the
/// prediction already in `dst` at `at` with row stride `stride`.
pub(super) fn add_transform(c: &[i16], dst: &mut [u8], at: usize, stride: usize) {
    if c.iter().all(|&v| v == 0) {
        return;
    }
    let mut t = [0i32; 16];
    for i in 0..4 {
        let (c0, c4, c8, c12) = (c[i] as i32, c[4 + i] as i32, c[8 + i] as i32, c[12 + i] as i32);
        let (a, b) = (c0 + c8, c0 - c8);
        let (cc, d) = (mul2(c4) - mul1(c12), mul1(c4) + mul2(c12));
        t[4 * i..4 * i + 4].copy_from_slice(&[a + d, b + cc, b - cc, a - d]);
    }
    for i in 0..4 {
        let dc = t[i] + 4;
        let (a, b) = (dc + t[8 + i], dc - t[8 + i]);
        let (cc, d) = (mul2(t[4 + i]) - mul1(t[12 + i]), mul1(t[4 + i]) + mul2(t[12 + i]));
        let row = at + i * stride;
        for (x, v) in [a + d, b + cc, b - cc, a - d].into_iter().enumerate() {
            let p = &mut dst[row + x];
            *p = (*p as i32 + (v >> 3)).clamp(0, 255) as u8;
        }
    }
}

/// Inverse Walsh-Hadamard transform of the second-order block into the DC
/// of each of the 16 luma blocks (RFC 6386 14.4).
pub(super) fn wht(inp: &[i16; 16], c: &mut [i16; 384]) {
    let mut t = [0i32; 16];
    for i in 0..4 {
        let at = |k: usize| inp[k + i] as i32;
        let (a0, a1) = (at(0) + at(12), at(4) + at(8));
        let (a2, a3) = (at(4) - at(8), at(0) - at(12));
        (t[i], t[8 + i], t[4 + i], t[12 + i]) = (a0 + a1, a0 - a1, a3 + a2, a3 - a2);
    }
    for i in 0..4 {
        let dc = t[4 * i] + 3;
        let (a0, a1) = (dc + t[4 * i + 3], t[4 * i + 1] + t[4 * i + 2]);
        let (a2, a3) = (t[4 * i + 1] - t[4 * i + 2], dc - t[4 * i + 3]);
        for (j, v) in [a0 + a1, a3 + a2, a0 - a1, a3 - a2].into_iter().enumerate() {
            c[64 * i + 16 * j] = (v >> 3) as i16;
        }
    }
}
