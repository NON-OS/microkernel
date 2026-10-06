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

//! The NØNOS mark: the brand's Ø from nonos-icon-cyan.svg, lit, with its
//! glow, as the recording on nonos.software shows it.

use super::glyph::glyph;
use super::style::mark_face;
use super::text::stamp;

/// The Ø's width and height at this screen class.
pub fn mark_size() -> (u32, u32) {
    mark_face().and_then(|f| glyph(&f, 0xD8)).map_or((0, 0), |g| (g.w, g.h))
}

/// Draw the Ø centred on (cx, cy) in `color`, its glow at `glow` / 256.
pub fn draw_mark(cx: u32, cy: u32, color: u32, glow: u32) {
    let Some(f) = mark_face() else { return };
    let Some(m) = glyph(&f, 0xD8) else { return };
    let (x, y) = (cx as i64 - m.w as i64 / 2, cy as i64 - m.h as i64 / 2);
    if glow > 0 {
        if let Some(h) = glyph(&f, 0x04) {
            stamp(x + h.left as i64, y + h.top as i64, &h, color, glow);
        }
    }
    stamp(x, y, &m, color, 256);
}
