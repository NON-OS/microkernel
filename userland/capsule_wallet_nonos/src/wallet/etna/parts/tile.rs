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

//! A quiet surface for a group of rows: surface fill, the tile corner and a
//! hairline. Each row is a name on the left in text-3 and a mono value on
//! the right.

use nonos_app_skeleton::PaintBuffer;

use super::super::rect::Rect;
use super::super::roles::Role;
use super::super::text::{draw, line, width};
use super::super::tokens::{LINE, RADIUS, SURFACE, TIGHT};
use super::rule::rule;

/// Rows sit 16 in from the tile's edge, 12 above and below their text.
pub const ROW_PAD_X: u32 = 16;

pub fn tile(fb: &mut PaintBuffer, at: Rect) {
    fb.fill_round(at.x, at.y, at.w, at.h, RADIUS, SURFACE);
    fb.stroke_round(at.x, at.y, at.w, at.h, RADIUS, 1, LINE);
}

pub fn row_height() -> u32 {
    (line(Role::RowName).max(line(Role::RowValue)) as u32) + 2 * TIGHT
}

/// One row at `y` inside `at`; a rule under it unless it is the last.
pub fn tile_row(fb: &mut PaintBuffer, at: Rect, y: u32, name: &str, value: &str, last: bool) {
    let top = (y + TIGHT) as i32;
    draw(fb, (at.x + ROW_PAD_X) as i32, top, Role::RowName, name);
    let vx = (at.x + at.w - ROW_PAD_X) as i32 - width(Role::RowValue, value);
    draw(fb, vx, top, Role::RowValue, value);
    if !last {
        rule(fb, at.x, y + row_height() - 1, at.w, ROW_PAD_X);
    }
}
