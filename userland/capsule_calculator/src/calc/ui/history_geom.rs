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

use nonos_toolkit::widgets::Rect;

use super::metrics::{metrics, px};
use super::rect::rect;
use crate::calc::history::CAP;

const ROW_H: i32 = 44;
const ROW_GAP: i32 = 6;
const PAD: i32 = 14;

/// The pane's inset around its rows, in drawing pixels.
pub fn pad() -> i32 {
    px(PAD)
}

pub fn top() -> i32 {
    let m = metrics();
    m.pane_pad + m.readout_h + m.pane_pad
}

pub fn pane(win_w: i32, win_h: i32) -> Rect {
    let (m, y) = (metrics(), top());
    rect(m.rail_w + m.pane_pad, y, win_w - m.rail_w - m.pane_pad * 2, win_h - y - m.pane_pad)
}

pub fn row(win_w: i32, i: usize) -> Rect {
    let (m, pad, row_h) = (metrics(), pad(), px(ROW_H));
    rect(
        m.rail_w + m.pane_pad + pad,
        top() + pad + i as i32 * (row_h + px(ROW_GAP)),
        win_w - m.rail_w - m.pane_pad * 2 - pad * 2,
        row_h,
    )
}

pub fn capacity(win_h: i32) -> usize {
    let (row_h, gap) = (px(ROW_H), px(ROW_GAP));
    let avail = win_h - metrics().pane_pad - top() - pad() * 2;
    if avail < row_h {
        return 0;
    }
    (((avail + gap) / (row_h + gap)) as usize).min(CAP)
}

/// Rows that can be scrolled past before the oldest entry sits on the last row.
pub fn max_scroll(len: usize, win_h: i32) -> usize {
    len.saturating_sub(capacity(win_h))
}

/// `scroll` moved by `rows` (negative is towards the newest), kept in range.
pub fn scroll_by(scroll: usize, rows: i32, len: usize, win_h: i32) -> usize {
    let moved = if rows < 0 {
        scroll.saturating_sub(rows.unsigned_abs() as usize)
    } else {
        scroll.saturating_add(rows as usize)
    };
    moved.min(max_scroll(len, win_h))
}

/// The ring index of the entry drawn on visible row `row`.
pub fn entry_at(scroll: usize, row: usize) -> usize {
    scroll + row
}
