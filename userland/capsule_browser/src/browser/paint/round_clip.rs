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

use alloc::vec;
use alloc::vec::Vec;

use super::corners::{corners, inset};

/* A clip rectangle rounded by corner radii `r` (top-left, top-right,
 * bottom-right, bottom-left), as plain rectangles: one row at a time where
 * a corner curves, one band for the rows between. A square clip, or none,
 * is itself. Painting through each in turn paints the rounded shape. */
pub(super) fn bands(clip: Option<[i32; 4]>, r: [u16; 4]) -> Vec<Option<[i32; 4]>> {
    let Some([x0, y0, x1, y1]) = clip.filter(|_| r != [0; 4]) else { return vec![clip] };
    let r = corners(r, x1 - x0, y1 - y0);
    let (top, bottom) = (r[0].max(r[1]), r[2].max(r[3]));
    let mut out = Vec::with_capacity((top + bottom + 1) as usize);
    for i in 0..top {
        out.push(Some([x0 + inset(r[0], i), y0 + i, x1 - inset(r[1], i), y0 + i + 1]));
    }
    out.push(Some([x0, y0 + top, x1, y1 - bottom]));
    for i in 0..bottom {
        out.push(Some([x0 + inset(r[3], i), y1 - 1 - i, x1 - inset(r[2], i), y1 - i]));
    }
    out
}
