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

//! Block elements filled as exact parts of the cell.

use nonos_app_skeleton::PaintBuffer;

/// Fill the part of the cell at `(x, y)`, sized `w` by `h`, that the block
/// element `ch` covers. False for any other character, so the caller draws
/// the glyph instead.
pub fn fill(fb: &mut PaintBuffer, ch: char, x: u32, y: u32, w: u32, h: u32, argb: u32) -> bool {
    /*
     * The face's glyphs miss the cell's edges by a pixel, which left a line
     * of the other colour between rows of half blocks. Each element is its
     * span of the cell in eighths: left, top, right, bottom.
     */
    let n = ch as u32;
    let (l, t, r, b) = match n {
        0x2580 => (0, 0, 8, 4),
        0x2581..=0x2588 => (0, 0x2588 - n, 8, 8),
        0x2589..=0x258F => (0, 0, 0x2590 - n, 8),
        0x2590 => (4, 0, 8, 8),
        0x2594 => (0, 0, 8, 1),
        0x2595 => (7, 0, 8, 8),
        0x2596..=0x259F => return quadrants(fb, n, x, y, w, h, argb),
        _ => return false,
    };
    part(fb, (x, y, w, h), (l, t, r, b), argb);
    true
}

/* Quadrant bits: upper left 1, upper right 2, lower left 4, lower right 8. */
const QUADRANTS: [u8; 10] = [4, 8, 1, 13, 9, 7, 11, 2, 6, 14];

fn quadrants(fb: &mut PaintBuffer, n: u32, x: u32, y: u32, w: u32, h: u32, argb: u32) -> bool {
    let bits = QUADRANTS[(n - 0x2596) as usize];
    for (bit, span) in [(1, (0, 0, 4, 4)), (2, (4, 0, 8, 4)), (4, (0, 4, 4, 8)), (8, (4, 4, 8, 8))]
    {
        if bits & bit != 0 {
            part(fb, (x, y, w, h), span, argb);
        }
    }
    true
}

fn part(fb: &mut PaintBuffer, cell: (u32, u32, u32, u32), span: (u32, u32, u32, u32), argb: u32) {
    let (x, y, w, h) = cell;
    let (l, t, r, b) = span;
    let (x0, x1) = (x + w * l / 8, x + w * r / 8);
    let (y0, y1) = (y + h * t / 8, y + h * b / 8);
    fb.fill_rect(x0, y0, x1 - x0, y1 - y0, argb);
}
