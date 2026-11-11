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

//! Where the videos sit on a page: the grid's tiles, and the rows of the
//! shared Table, placed as the Table places them, so the click, the hover
//! and the scroll count the rows it draws.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::TABLE_ROW_H;
use nonos_toolkit::widgets::Rect;

pub const TILE_W: u32 = 214;
pub const TILE_H: u32 = 186;
pub const GAP: u32 = 16;
/// Room kept right of the list for its scroll bar, and the bar's width.
const SCROLL_ROOM: u32 = 12;
const SCROLL_W: u32 = 6;

/// A tile's width, height and the gap between tiles, in drawing pixels.
fn tile() -> (u32, u32, u32) {
    let s = scale();
    (s.px(TILE_W), s.px(TILE_H), s.px(GAP))
}

pub fn columns(w: u32) -> usize {
    let (tw, _, gap) = tile();
    (((w + gap) / (tw + gap)) as usize).max(1)
}

pub fn tile_rect(body: Rect, slot: usize) -> Rect {
    let ((tw, th, gap), cols) = (tile(), columns(body.w));
    let (col, row) = ((slot % cols) as u32, (slot / cols) as u32);
    let (x, y) = (body.x + col * (tw + gap), body.y + row * (th + gap));
    Rect { x, y, w: tw, h: th }
}

pub fn grid_capacity(body: Rect) -> usize {
    let (_, th, gap) = tile();
    let rows = ((body.h + gap) / (th + gap)).max(1) as usize;
    rows * columns(body.w)
}

/// The Table in `area`, short of the scroll bar's room.
pub fn list_at(area: Rect) -> Rect {
    Rect { w: area.w.saturating_sub(scale().px(SCROLL_ROOM)), ..area }
}

/// The scroll bar's track, down the right of `area`.
pub fn track(area: Rect) -> Rect {
    let w = scale().px(SCROLL_W);
    Rect { x: area.x + area.w.saturating_sub(w), w: w.min(area.w), ..area }
}

/// The header's height, which is also each row's, and the rule under it.
fn row_h() -> u32 {
    scale().px(TABLE_ROW_H)
}

/// How many whole rows the Table `at` draws under its header and rule.
pub fn rows_visible(at: Rect) -> usize {
    let head = row_h() + scale().hair();
    (at.h.saturating_sub(head) / row_h().max(1)) as usize
}

/// The `k`th row the Table `at` draws.
pub fn row_rect(at: Rect, k: usize) -> Rect {
    let y = at.y + row_h() + scale().hair() + k as u32 * row_h();
    Rect { y, h: row_h(), ..at }
}
