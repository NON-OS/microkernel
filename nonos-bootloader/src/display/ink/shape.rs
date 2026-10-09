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

//! Rounded rectangles with anti-aliased corners, blended over the screen.

use crate::display::fx::mix;
use crate::display::gop::{fill_rect, get_pixel, put_pixel};

/// A filled rectangle with corners of radius `r`.
pub fn round_rect(x: u32, y: u32, w: u32, h: u32, r: u32, color: u32) {
    let r = r.min(w / 2).min(h / 2);
    if r == 0 {
        fill_rect(x, y, w, h, color);
        return;
    }
    fill_rect(x, y + r, w, h - 2 * r, color);
    fill_rect(x + r, y, w - 2 * r, r, color);
    fill_rect(x + r, y + h - r, w - 2 * r, r, color);
    for dy in 0..r {
        for dx in 0..r {
            let a = corner(dx, dy, r);
            if a == 0 {
                continue;
            }
            let (l, t) = (x + dx, y + dy);
            let (rr, b) = (x + w - 1 - dx, y + h - 1 - dy);
            for (px, py) in [(l, t), (rr, t), (l, b), (rr, b)] {
                let c = if a >= 255 { color } else { mix(get_pixel(px, py), color, a) };
                put_pixel(px, py, c);
            }
        }
    }
}

/// Coverage, 0 to 255, of the corner pixel `dx`, `dy` in from the corner of
/// a radius `r` arc: 4x4 samples against the circle centred at (r, r).
fn corner(dx: u32, dy: u32, r: u32) -> u32 {
    let r4 = (r * 8) as i64;
    let mut hits = 0u32;
    for sy in 0..4 {
        for sx in 0..4 {
            let px = (dx * 8 + sx * 2 + 1) as i64 - r4;
            let py = (dy * 8 + sy * 2 + 1) as i64 - r4;
            if px * px + py * py <= r4 * r4 {
                hits += 1;
            }
        }
    }
    hits * 255 / 16
}
