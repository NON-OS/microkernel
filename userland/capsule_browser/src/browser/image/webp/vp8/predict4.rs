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

use super::avg::avg3;

/// A 4x4 block predicted by `mode` (RFC 6386 12.3) from `t`, the corner
/// then the eight pixels above and above-right, and `l`, the four to the
/// left. Returns the block in raster order.
pub(super) fn pred4(mode: u8, t: &[u8; 9], l: &[u8; 4]) -> [u8; 16] {
    let x = t[0];
    let a = |i: usize| t[1 + i];
    let e = [x, l[0], l[1], l[2], l[3]];
    let mut o = [0u8; 16];
    let mut put = |c: usize, r: usize, v: u8| o[r * 4 + c] = v;
    match mode {
        1 => {
            for (r, &lr) in l.iter().enumerate() {
                for c in 0..4 {
                    put(c, r, (lr as i32 + a(c) as i32 - x as i32).clamp(0, 255) as u8);
                }
            }
        }
        2 => (0..16).for_each(|i| put(i % 4, i / 4, avg3(t[i % 4], a(i % 4), a(i % 4 + 1)))),
        3 => {
            let rows = [
                avg3(x, l[0], l[1]),
                avg3(l[0], l[1], l[2]),
                avg3(l[1], l[2], l[3]),
                avg3(l[2], l[3], l[3]),
            ];
            (0..16).for_each(|i| put(i % 4, i / 4, rows[i / 4]));
        }
        4 => {
            /* Down-right: the edge from bottom-left up through the corner. */
            let edge = [e[4], e[3], e[2], e[1], x, a(0), a(1), a(2), a(3)];
            (0..16).for_each(|i| {
                let k = 3 + i % 4 - i / 4;
                put(i % 4, i / 4, avg3(edge[k], edge[k + 1], edge[k + 2]));
            });
        }
        6 => (0..16).for_each(|i| {
            let k = i % 4 + i / 4;
            put(i % 4, i / 4, avg3(a(k), a(k + 1), a((k + 2).min(7))));
        }),
        5 | 7 | 8 | 9 => super::predict4_ext::pred4_ext(mode, t, l, &mut o),
        _ => {
            let s: u32 = (0..4).map(|i| a(i) as u32 + l[i] as u32).sum();
            o = [((s + 4) >> 3) as u8; 16];
        }
    }
    o
}
