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
//! The frame's measures at one to one (the design notes, Window
//! The window controls: a line glyph each, no colour at rest. The hovered
//! one takes a wash, and close a red fill, so its meaning is said by its
//! glyph and by its colour, never by colour alone.

use super::control_rect::{control_rect_at, CONTROLS};
use super::glyph;
use super::hit_test::DecorationHit;
use super::palette::{control_wash, glyph_hover, glyph_rest, CLOSE_HOVER, ON_CLOSE_HOVER};
use super::scale::at;
use crate::paint::PaintBuffer;

/// The wash's corners, inside the target.
const WASH_RADIUS: u32 = 6;

pub fn draw_controls_at(
    fb: &mut PaintBuffer,
    maximized: bool,
    hover: DecorationHit,
    quarters: u32,
) {
    let (w, h) = (fb.width, fb.height);
    for (i, which) in CONTROLS.iter().enumerate() {
        let r = control_rect_at(i as u32, w, h, maximized, quarters);
        let lit = hover == *which;
        let ink = match (lit, *which) {
            (true, DecorationHit::CloseButton) => {
                fb.fill_round(r.x, r.y, r.w, r.h, at(WASH_RADIUS, quarters), CLOSE_HOVER);
                ON_CLOSE_HOVER
            }
            (true, _) => {
                fb.fill_round(r.x, r.y, r.w, r.h, at(WASH_RADIUS, quarters), control_wash());
                glyph_hover()
            }
            _ => glyph_rest(),
        };
        glyph::draw(fb, *which, r.x + r.w / 2, r.y + r.h / 2, quarters, ink);
    }
}
