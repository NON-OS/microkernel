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

use alloc::vec::Vec;

use super::cursor::Cursor;

/// One outline point of a simple glyph, in font units.
pub(super) struct Point {
    pub(super) x: i32,
    pub(super) y: i32,
    pub(super) on: bool,
}

/// A simple glyph's points from its flag bytes and triplet stream (WOFF2
/// 5.2): seven flag bits size the x and y steps, bit 7 marks off-curve.
pub(super) fn decode(flags: &[u8], data: &mut Cursor) -> Option<Vec<Point>> {
    let mut pts = Vec::new();
    pts.try_reserve_exact(flags.len()).ok()?;
    let (mut x, mut y) = (0i32, 0i32);
    for &f in flags {
        let t = (f & 0x7f) as i32;
        let len = match t {
            0..=83 => 1,
            84..=119 => 2,
            120..=123 => 3,
            _ => 4,
        };
        let raw = data.bytes(len)?;
        let b = |i: usize| raw[i] as i32;
        let (dx, dy) = match t {
            0..=9 => (0, sign(t, ((t & 14) << 7) + b(0))),
            10..=19 => (sign(t, (((t - 10) & 14) << 7) + b(0)), 0),
            20..=83 => {
                let k = t - 20;
                let dx = 1 + (k & 0x30) + (b(0) >> 4);
                (sign(t, dx), sign(t >> 1, 1 + ((k & 0x0c) << 2) + (b(0) & 0x0f)))
            }
            84..=119 => {
                let (k, m) = (t - 84, (t - 84) % 12);
                (sign(t, 1 + ((k / 12) << 8) + b(0)), sign(t >> 1, 1 + ((m >> 2) << 8) + b(1)))
            }
            120..=123 => {
                (sign(t, (b(0) << 4) + (b(1) >> 4)), sign(t >> 1, ((b(1) & 15) << 8) + b(2)))
            }
            _ => (sign(t, (b(0) << 8) + b(1)), sign(t >> 1, (b(2) << 8) + b(3))),
        };
        (x, y) = (x.checked_add(dx)?, y.checked_add(dy)?);
        pts.push(Point { x, y, on: f & 0x80 == 0 });
    }
    Some(pts)
}

/// The step `v` with the sign bit 0 of `flag` gives: set is positive.
fn sign(flag: i32, v: i32) -> i32 {
    if flag & 1 != 0 {
        v
    } else {
        -v
    }
}
