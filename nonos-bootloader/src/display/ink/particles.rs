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

//! The mark assembling: particles drift in and settle on the Ø's strokes, as
//! on nonos.software. One frame per call; the caller paces and clears.

use super::glyph::{coverage, glyph};
use super::style::{mark_face, unit};
use crate::display::fx::mix;
use crate::display::gop::{fill_rect, get_pixel};

const COUNT: u32 = 420;

/// Draw the particles at `t` thousandths of the way home, around the Ø
/// centred on (cx, cy), scattered at most `spread` pixels from it.
pub fn draw_particles(cx: u32, cy: u32, spread: u32, t: u32, color: u32) {
    let Some(f) = mark_face() else { return };
    let Some(g) = glyph(&f, 0xD8) else { return };
    let ink: u32 = (0..g.h)
        .flat_map(|y| (0..g.w).map(move |x| (x, y)))
        .filter(|&(x, y)| coverage(&g, x, y) > 8)
        .count() as u32;
    if ink == 0 {
        return;
    }
    let e = ease(t.min(1000)) as i64;
    let size = (unit() / 3).max(2);
    let (ox, oy) = (cx as i64 - g.w as i64 / 2, cy as i64 - g.h as i64 / 2);
    let mut seen = 0u32;
    let mut next = 0u32;
    for y in 0..g.h {
        for x in 0..g.w {
            if coverage(&g, x, y) <= 8 {
                continue;
            }
            if seen == next * ink / COUNT {
                let h = hash(next);
                let sx = cx as i64 + ((h & 0xFFFF) as i64 * 2 - 0xFFFF) * spread as i64 / 0xFFFF;
                let sy =
                    cy as i64 + (((h >> 16) & 0xFFFF) as i64 * 2 - 0xFFFF) * spread as i64 / 0xFFFF;
                let px = sx + (ox + x as i64 - sx) * e / 1000;
                let py = sy + (oy + y as i64 - sy) * e / 1000;
                let a = (120 + e * 135 / 1000) as u32 * (60 + (h >> 24) % 196) / 255;
                if px > 0 && py > 0 {
                    let (px, py) = (px as u32, py as u32);
                    fill_rect(px, py, size, size, mix(get_pixel(px, py), color, a));
                }
                next += 1;
            }
            seen += 1;
        }
    }
}

/// Ease out: fast at first, settling at the end.
const fn ease(t: u32) -> u32 {
    1000 - (1000 - t) * (1000 - t) / 1000 * (1000 - t) / 1000
}

const fn hash(i: u32) -> u32 {
    (i.wrapping_mul(0x9E37_79B1) ^ 0x85EB_CA77).wrapping_mul(0x2C1B_3C6D) >> 3
}
