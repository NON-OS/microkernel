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

use super::modes::{HE, TM, VE};

/// The edge a whole-block predictor reads: `top` and `left` (n each), the
/// corner, and whether the frame has a row above and a column left of it
/// (DC averages only the sides that exist, 128 with neither).
pub(super) struct Edge<'a> {
    pub top: &'a [u8],
    pub left: &'a [u8],
    pub corner: u8,
    pub has_top: bool,
    pub has_left: bool,
}

/// A 16x16 luma or 8x8 chroma block predicted by DC, V, H or TM
/// (RFC 6386 12.2) into `out`, n x n in raster order.
pub(super) fn pred_block(mode: u8, e: &Edge, out: &mut [u8]) {
    let n = e.top.len();
    let shift = n.trailing_zeros();
    let sum = |s: &[u8]| s.iter().map(|&v| v as u32).sum::<u32>();
    match mode {
        VE => out.chunks_mut(n).for_each(|r| r.copy_from_slice(e.top)),
        HE => out.chunks_mut(n).zip(e.left).for_each(|(r, &l)| r.fill(l)),
        TM => {
            for (y, r) in out.chunks_mut(n).enumerate() {
                for (x, o) in r.iter_mut().enumerate() {
                    let v = e.left[y] as i32 + e.top[x] as i32 - e.corner as i32;
                    *o = v.clamp(0, 255) as u8;
                }
            }
        }
        _ => {
            let dc = match (e.has_top, e.has_left) {
                (true, true) => (sum(e.top) + sum(e.left) + n as u32) >> (shift + 1),
                (true, false) => (sum(e.top) + (n as u32 >> 1)) >> shift,
                (false, true) => (sum(e.left) + (n as u32 >> 1)) >> shift,
                (false, false) => 128,
            };
            out.fill(dc as u8);
        }
    }
}
