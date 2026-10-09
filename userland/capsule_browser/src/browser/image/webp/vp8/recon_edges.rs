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

use super::planes::Plane;
use super::predict::{pred_block, Edge};

/// Predict an n x n block (16 luma or 8 chroma) with a whole-block mode.
pub(super) fn whole(
    p: &mut Plane,
    (x0, y0): (usize, usize),
    n: usize,
    mode: u8,
    (has_left, has_top): (bool, bool),
) {
    let (x, y) = (x0 as isize, y0 as isize);
    let (top, left, corner) = (p.row(x, y - 1, n), p.col(x - 1, y, n), p.at(x - 1, y - 1));
    let mut out = [0u8; 256];
    let e = Edge { top: &top[..n], left: &left[..n], corner, has_top, has_left };
    pred_block(mode, &e, &mut out[..n * n]);
    p.put(x0, y0, n, &out[..n * n]);
}

/* The four pixels right of the macroblock on the row above it, which every
 * 4x4 block of its right column predicts from: 127 on the first row,
 * the last pixel above repeated at the frame's right edge. */
pub(super) fn top_right(y: &Plane, mbx: usize, mby: usize, mbw: usize) -> [u8; 4] {
    if mby == 0 {
        return [127; 4];
    }
    let row = (mby * 16 - 1) as isize;
    if mbx + 1 >= mbw {
        return [y.at((mbx * 16 + 15) as isize, row); 4];
    }
    let r = y.row((mbx * 16 + 16) as isize, row, 4);
    [r[0], r[1], r[2], r[3]]
}
