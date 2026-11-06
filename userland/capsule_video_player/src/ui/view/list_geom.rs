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

pub fn columns(w: u32) -> usize {
    (((w + GAP) / (TILE_W + GAP)) as usize).max(1)
}

pub fn tile_rect(body: Rect, slot: usize) -> Rect {
    let cols = columns(body.w);
    let (col, row) = ((slot % cols) as u32, (slot / cols) as u32);
    let (x, y) = (body.x + col * (TILE_W + GAP), body.y + row * (TILE_H + GAP));
    Rect { x, y, w: TILE_W, h: TILE_H }
}

pub fn grid_capacity(body: Rect) -> usize {
    let rows = ((body.h + 16) / (TILE_H + 16)).max(1) as usize;
    rows * columns(body.w)
}

/// The Table in `area`, short of the scroll bar's room.
pub fn list_at(area: Rect) -> Rect {
    Rect { w: area.w.saturating_sub(SCROLL_ROOM), ..area }
}

/// The scroll bar's track, down the right of `area`.
pub fn track(area: Rect) -> Rect {
    Rect { x: area.x + area.w.saturating_sub(SCROLL_W), w: SCROLL_W.min(area.w), ..area }
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
