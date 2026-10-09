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

use nonos_app_skeleton::PaintBuffer;

use super::corners::{corners, inset};
use super::fill_page::fill_page;

/* Rounded rect with a radius per corner (top-left, top-right, bottom-right,
 * bottom-left): one full-width band between the corner rows, then per-row
 * spans whose insets follow each corner's circle. A radius past half the
 * shorter side shrinks to it, so a 50% radius on a square draws a circle
 * and on a wide box a pill. */
pub(super) fn fill_rounded(
    fb: &mut PaintBuffer,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    radius: [u16; 4],
    color: u32,
    clip: Option<[i32; 4]>,
) {
    let r = corners(radius, w, h);
    if r == [0; 4] {
        fill_page(fb, x, y, w, h, color, clip);
        return;
    }
    let (top, bot) = (r[0].max(r[1]), r[2].max(r[3]));
    fill_page(fb, x, y + top, w, h - top - bot, color, clip);
    for row in 0..top {
        let (l, rr) = (inset(r[0], row), inset(r[1], row));
        fill_page(fb, x + l, y + row, w - l - rr, 1, color, clip);
    }
    for row in 0..bot {
        let (l, rr) = (inset(r[3], row), inset(r[2], row));
        fill_page(fb, x + l, y + h - 1 - row, w - l - rr, 1, color, clip);
    }
}
