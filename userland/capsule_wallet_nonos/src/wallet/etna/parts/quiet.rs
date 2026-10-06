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

//! The actions a screen offers without asking for them: rows of mono 14 in
//! one outlined box, with rules inset under the text, like a settings list.

use nonos_app_skeleton::PaintBuffer;

use super::super::rect::Rect;
use super::super::roles::Role;
use super::super::text::{draw_in, line};
use super::super::tokens::{CORNER, OUTLINE, TEXT, TEXT_3};
use super::rule::rule;

const PAD_X: u32 = 16;
const PAD_Y: u32 = 14;

pub fn quiet_row_height() -> u32 {
    line(Role::RowValue) as u32 + 2 * PAD_Y
}

/// Draw the group; each row's rectangle is written to `hits` for clicks.
pub fn quiet_group(
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    w: u32,
    rows: &[(&str, bool)],
    hits: &mut [Rect],
) -> u32 {
    let rh = quiet_row_height();
    for (i, (title, enabled)) in rows.iter().enumerate() {
        let ry = y + rh * i as u32;
        let ink = if *enabled { TEXT } else { TEXT_3 };
        draw_in(fb, (x + PAD_X) as i32, (ry + PAD_Y) as i32, Role::RowValue, title, ink);
        if i + 1 < rows.len() {
            rule(fb, x, ry + rh - 1, w, PAD_X);
        }
        if let Some(slot) = hits.get_mut(i) {
            *slot = Rect::new(x, ry, w, rh);
        }
    }
    let h = rh * rows.len() as u32;
    fb.stroke_round(x, y, w, h, CORNER, 1, OUTLINE);
    h
}
