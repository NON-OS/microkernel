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

//! Where the pointer lands for the device's cursor plane (CURSOR, section
//! 12): on the screen, from the canvas clients point on, and on a mirrored
//! output, inside the box the screen is fitted and centred in there; and
//! the CURSOR a position becomes. The arrow's tip is its hot spot.

use nonos_gfx_wire::body::cursor::{NEW_SHAPE, VISIBLE};
use nonos_gfx_wire::Cursor;

use crate::sw_blitter::resample::fit_box;

/// Canvas point `at` on the screen: times the scale (`q` quarters), or
/// shrunk as the floor canvas is when the canvas is the larger.
pub fn on_screen(at: (u32, u32), canvas: (u32, u32), screen: (u32, u32), q: u32) -> (i32, i32) {
    let shrunk = canvas.0 > screen.0 || canvas.1 > screen.1;
    let f = |v: u32, c: u32, s: u32| {
        let v = v as u64;
        let p = if shrunk { v * s as u64 / c.max(1) as u64 } else { v * q.max(4) as u64 / 4 };
        p.min(i32::MAX as u64) as i32
    };
    (f(at.0, canvas.0, screen.0), f(at.1, canvas.1, screen.1))
}

/// Screen point `p` on an output `head` big that mirrors the screen.
pub fn on_head(p: (i32, i32), screen: (u32, u32), head: (u32, u32)) -> (i32, i32) {
    let (ox, oy, bw, bh) = fit_box(screen, head);
    let f = |v: i32, s: u32, b: u32, o: u32| {
        let v = v.max(0) as u64 * b as u64 / s.max(1) as u64;
        (o as u64 + v).min(i32::MAX as u64) as i32
    };
    (f(p.0, screen.0, bw, ox), f(p.1, screen.1, bh, oy))
}

/// CURSOR for the arrow `side` big at `p`; `new_shape` when its slot was
/// just drawn.
pub fn message(
    p: (i32, i32),
    visible: bool,
    new_shape: bool,
    side: (u16, u16),
    hot: (u16, u16),
) -> Cursor {
    let flags = if visible { VISIBLE } else { 0 } | if new_shape { NEW_SHAPE } else { 0 };
    let (x, y, width, height) = (p.0, p.1, side.0, side.1);
    Cursor { flags, x, y, hot_x: hot.0, hot_y: hot.1, width, height }
}
