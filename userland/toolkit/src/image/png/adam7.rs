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

use crate::image::types::ImageSize;

/* Pass geometry as [x0, y0, dx, dy]: where a pass starts and how far apart
 * its pixels lie. Adam7 has seven passes; a plain image is one pass. */
pub(super) const ADAM7: [[usize; 4]; 7] = [
    [0, 0, 8, 8],
    [4, 0, 8, 8],
    [0, 4, 4, 8],
    [2, 0, 4, 4],
    [0, 2, 2, 4],
    [1, 0, 2, 2],
    [0, 1, 1, 2],
];
pub(super) const PLAIN: [[usize; 4]; 1] = [[0, 0, 1, 1]];

/* Pixels per row and number of rows a pass holds in a w x h image. A pass
 * with either count zero has no scanlines, not even filter bytes. */
pub(super) fn pass_size(p: [usize; 4], w: usize, h: usize) -> (usize, usize) {
    let cols = if w > p[0] { (w - p[0]).div_ceil(p[2]) } else { 0 };
    let rows = if h > p[1] { (h - p[1]).div_ceil(p[3]) } else { 0 };
    (cols, rows)
}

/* The first pass at or after `p` that holds any pixels, with its pixels per
 * row and row count; `passes.len()` once none is left. */
pub(super) fn next_pass(
    passes: &[[usize; 4]],
    mut p: usize,
    size: ImageSize,
) -> (usize, (usize, usize)) {
    while p < passes.len() {
        let s = pass_size(passes[p], size.width as usize, size.height as usize);
        if s.0 > 0 && s.1 > 0 {
            return (p, s);
        }
        p += 1;
    }
    (p, (0, 0))
}
