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

use super::look::{look, Look, Shows};
use crate::browser::paint::glyph::{self, Glyph};
use crate::browser::paint::tokens::palette;
use crate::browser::ui::rect::Rect;
use crate::browser::ui::settings_geom::Control;

const RADIUS: u32 = 10;
const ICON: i32 = 16;

/* The text size's Smaller and Larger steps: a minus and a plus, dimmed
 * at the smallest and largest size. */
pub fn paint(fb: &mut PaintBuffer, s: &Shows, c: Control, r: Rect) {
    let p = palette();
    let (fill, edge, ink) = match look(s, c) {
        Look::Dim => (p.field, p.field, p.disabled),
        _ => (p.surface, p.edge, p.ink),
    };
    let (x, y, w, h) = (r.x as u32, r.y as u32, r.w as u32, r.h as u32);
    fb.fill_round(x, y, w, h, RADIUS, fill);
    fb.stroke_round(x, y, w, h, RADIUS, 1, edge);
    let (cx, cy) = (r.x + r.w / 2, r.y + r.h / 2);
    match c {
        Control::Larger => glyph::draw(fb, Glyph::Plus, cx, cy, ICON, ink),
        _ => fb.fill_rect((cx - ICON / 3) as u32, (cy - 1) as u32, (2 * ICON / 3) as u32, 2, ink),
    }
}
