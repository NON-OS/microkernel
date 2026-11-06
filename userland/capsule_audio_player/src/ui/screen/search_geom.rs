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

//! Where the Search page puts its Clear and Play now buttons, its table head
//! and its rows, which the painter and the target table both read.

use nonos_toolkit::widgets::text;

use super::super::geometry::Rect;
use super::super::metrics::{line_h, BODY, ITEM, PAGE, S3, S4};
use super::lib_geom::ROW_H;

pub const CLEAR: &str = "Clear";
pub const PLAY: &str = "Play now";
pub const DOWNLOAD: &str = "Download";

/// A button's width for `label` after its glyph, as the player sized them.
fn glyph_w(label: &str) -> i32 {
    text::width(label, ITEM) as i32 + ITEM as i32 + S3 + S3 * 4
}

pub fn clear_rect(r: Rect) -> Rect {
    let w = glyph_w(CLEAR);
    Rect::new(r.right() - w, r.y + (line_h(PAGE) - 38) / 2, w, 38)
}

pub fn play_rect(r: Rect) -> Rect {
    let c = clear_rect(r);
    // One place for Play now and, with an address in the field, Download.
    let w = glyph_w(PLAY).max(glyph_w(DOWNLOAD));
    Rect::new(c.x - S4 - w, c.y, w, c.h)
}

pub fn head_rect(r: Rect) -> Rect {
    Rect::new(r.x, r.y + line_h(PAGE) + line_h(BODY) + S3 + S4, r.w, 28)
}

pub fn list_rect(r: Rect) -> Rect {
    let top = head_rect(r).bottom() + S3;
    Rect::new(r.x, top, r.w, (r.bottom() - top).max(0))
}

pub fn visible(r: Rect) -> usize {
    (list_rect(r).h / ROW_H).max(0) as usize
}
