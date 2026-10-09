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

use crate::image::jpeg::idct::idct_8x8;

/* C(u)/2 cos((2x+1)u pi / 2k) for the reduced 4- and 2-point transforms:
 * the first k coefficients of an 8-point block give its k-sample picture
 * directly, the DC term staying the block mean. */
const T4: [[f32; 4]; 4] = [
    [0.353_553_4; 4],
    [0.461_939_8, 0.191_341_7, -0.191_341_7, -0.461_939_8],
    [0.353_553_4, -0.353_553_4, -0.353_553_4, 0.353_553_4],
    [0.191_341_7, -0.461_939_8, 0.461_939_8, -0.191_341_7],
];
const T2: [[f32; 2]; 2] = [[0.353_553_4; 2], [0.353_553_4, -0.353_553_4]];

/// Dequantize a kept `k` x `k` block (`q` in natural order) and inverse
/// transform it into `k` x `k` level-shifted samples written to `out` at
/// row stride `stride`.
pub fn idct_scaled(coef: &[i16], q: &[u16; 64], k: usize, out: &mut [u8], stride: usize) {
    let deq = |r: usize, c: usize| coef[r * k + c] as i32 * q[r * 8 + c] as i32;
    match k {
        8 => {
            let mut f = [0i32; 64];
            for (i, v) in f.iter_mut().enumerate() {
                *v = deq(i / 8, i % 8);
            }
            let mut s = [0u8; 64];
            idct_8x8(&f, &mut s);
            for r in 0..8 {
                out[r * stride..r * stride + 8].copy_from_slice(&s[r * 8..r * 8 + 8]);
            }
        }
        4 => reduced(&T4, deq, out, stride),
        2 => reduced(&T2, deq, out, stride),
        _ => out[0] = level(deq(0, 0) as f32 / 8.0),
    }
}

fn reduced<const K: usize>(
    t: &[[f32; K]; K],
    deq: impl Fn(usize, usize) -> i32,
    out: &mut [u8],
    stride: usize,
) {
    let mut rows = [[0f32; K]; K];
    for (v, row) in rows.iter_mut().enumerate() {
        for (x, o) in row.iter_mut().enumerate() {
            *o = (0..K).map(|u| deq(v, u) as f32 * t[u][x]).sum();
        }
    }
    for y in 0..K {
        for x in 0..K {
            let s: f32 = (0..K).map(|v| rows[v][x] * t[v][y]).sum();
            out[y * stride + x] = level(s);
        }
    }
}

fn level(v: f32) -> u8 {
    (v + 128.5).clamp(0.0, 255.0) as u8
}
