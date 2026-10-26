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

//! The NONOS block banner, the art the build prints, drawn from its full
//! blocks as rectangles: no glyph, so nothing can come out as tofu or smear.

use nonos_app_skeleton::PaintBuffer;

use crate::term::theme::types::Theme;

pub(crate) const BANNER: [&str; 6] = [
    "███╗   ██╗ ██████╗ ███╗   ██╗ ██████╗ ███████╗",
    "████╗  ██║██╔═══██╗████╗  ██║██╔═══██╗██╔════╝",
    "██╔██╗ ██║██║   ██║██╔██╗ ██║██║   ██║███████╗",
    "██║╚██╗██║██║   ██║██║╚██╗██║██║   ██║╚════██║",
    "██║ ╚████║╚██████╔╝██║ ╚████║╚██████╔╝███████║",
    "╚═╝  ╚═══╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ ╚══════╝",
];

const CELL_W: i32 = 8; // one block of the art, in pixels
const CELL_H: i32 = 15;
const DEPTH: i32 = 3; // how far the side falls back, down and to the right

// Draw the banner at (x, y), inside `right`, as the emblem is drawn: the face
// lit from above, the side in shadow in place of the art's box strokes.
// Returns the y below it, or `y` unchanged when too wide and left out.
pub fn draw_banner(fb: &mut PaintBuffer, x: i32, y: i32, right: i32, t: &Theme) -> i32 {
    let cols = BANNER.iter().map(|l| l.chars().count()).max().unwrap_or(0) as i32;
    if x + cols * CELL_W + DEPTH > right || x < 0 || y < 0 {
        return y;
    }
    let side = mix(t.accent, 0xFF00_0000, 150);
    for pass in 0..2 {
        for (r, line) in BANNER.iter().enumerate() {
            let top = mix(t.accent, 0xFFFF_FFFF, 90 - r as u32 * 18);
            for (c, ch) in line.chars().enumerate() {
                if ch != '\u{2588}' {
                    continue;
                }
                let (cx, cy) = (x + c as i32 * CELL_W, y + r as i32 * CELL_H);
                if pass == 0 {
                    fb.fill_rect(
                        (cx + DEPTH) as u32,
                        (cy + DEPTH) as u32,
                        CELL_W as u32,
                        CELL_H as u32,
                        side,
                    );
                } else {
                    fb.fill_rect(cx as u32, cy as u32, CELL_W as u32, CELL_H as u32, top);
                }
            }
        }
    }
    y + BANNER.len() as i32 * CELL_H + DEPTH
}

/// `a` toward `b` by `k` of 255, opaque.
fn mix(a: u32, b: u32, k: u32) -> u32 {
    let ch = |s: u32| ((a >> s & 0xFF) * (255 - k) + (b >> s & 0xFF) * k) / 255;
    0xFF00_0000 | ch(16) << 16 | ch(8) << 8 | ch(0)
}
