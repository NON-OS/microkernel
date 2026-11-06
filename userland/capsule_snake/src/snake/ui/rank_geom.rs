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

use nonos_toolkit::widgets::rect::{contains, inset};
use nonos_toolkit::widgets::Rect;

use super::metrics::{FOOT_BTN_H, FOOT_BTN_W, GAP, PAD_TIGHT, RAIL_W, ROW_H, TABLE_HEAD_H};
use super::rect::{self, at};

pub const AWARD_ROWS: usize = 6;

fn stage(w: u32, h: u32) -> Rect {
    let c = rect::content(w, h);
    Rect { h: c.h.saturating_sub(FOOT_BTN_H + GAP), ..c }
}

pub fn table(w: u32, h: u32) -> Rect {
    let s = stage(w, h);
    Rect { w: s.w.saturating_sub(RAIL_W.min(s.w) + GAP), ..s }
}

pub fn awards(w: u32, h: u32) -> Rect {
    let s = stage(w, h);
    let rail_w = RAIL_W.min(s.w);
    at(s.x + s.w.saturating_sub(rail_w), s.y, rail_w, s.h)
}

/// Where the runs' table is drawn: its card less `PAD_TIGHT` all round.
pub fn sheet(w: u32, h: u32) -> Rect {
    inset(table(w, h), PAD_TIGHT)
}

pub fn award_row(w: u32, h: u32, index: usize) -> Rect {
    let inner = inset(awards(w, h), PAD_TIGHT);
    let top = inner.y + TABLE_HEAD_H;
    rect::row(at(inner.x, top, inner.w, ROW_H), index, ROW_H, 0)
}

pub fn back(w: u32, h: u32) -> Rect {
    let c = rect::content(w, h);
    let y = c.y + c.h.saturating_sub(FOOT_BTN_H);
    at(c.x, y, FOOT_BTN_W, FOOT_BTN_H)
}

pub fn back_at(w: u32, h: u32, x: i32, y: i32) -> bool {
    contains(back(w, h), x, y)
}

// The rest of the footer band, right of Back: where the screen says what
// became of the ranks on disk.
pub fn kept_note(w: u32, h: u32) -> Rect {
    let b = back(w, h);
    let c = rect::content(w, h);
    let x = b.x + b.w + GAP;
    at(x, b.y, (c.x + c.w).saturating_sub(x), b.h)
}
