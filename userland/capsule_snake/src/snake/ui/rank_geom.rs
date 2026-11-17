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

use super::metrics::metrics;
use super::rect::{self, at};

pub const AWARD_ROWS: usize = 6;

fn stage(w: u32, h: u32) -> Rect {
    let (c, m) = (rect::content(w, h), metrics());
    Rect { h: c.h.saturating_sub(m.foot_btn_h + m.gap), ..c }
}

pub fn table(w: u32, h: u32) -> Rect {
    let (s, m) = (stage(w, h), metrics());
    Rect { w: s.w.saturating_sub(m.rail_w.min(s.w) + m.gap), ..s }
}

pub fn awards(w: u32, h: u32) -> Rect {
    let s = stage(w, h);
    let rail_w = metrics().rail_w.min(s.w);
    at(s.x + s.w.saturating_sub(rail_w), s.y, rail_w, s.h)
}

/// Where the runs' table is drawn: its card less `PAD_TIGHT` all round.
pub fn sheet(w: u32, h: u32) -> Rect {
    inset(table(w, h), metrics().pad_tight)
}

pub fn award_row(w: u32, h: u32, index: usize) -> Rect {
    let m = metrics();
    let inner = inset(awards(w, h), m.pad_tight);
    let top = inner.y + m.table_head_h;
    rect::row(at(inner.x, top, inner.w, m.row_h), index, m.row_h, 0)
}

pub fn back(w: u32, h: u32) -> Rect {
    let (c, m) = (rect::content(w, h), metrics());
    let y = c.y + c.h.saturating_sub(m.foot_btn_h);
    at(c.x, y, m.foot_btn_w, m.foot_btn_h)
}

pub fn back_at(w: u32, h: u32, x: i32, y: i32) -> bool {
    contains(back(w, h), x, y)
}

// The rest of the footer band, right of Back: where the screen says what
// became of the ranks on disk.
pub fn kept_note(w: u32, h: u32) -> Rect {
    let b = back(w, h);
    let c = rect::content(w, h);
    let x = b.x + b.w + metrics().gap;
    at(x, b.y, (c.x + c.w).saturating_sub(x), b.h)
}
