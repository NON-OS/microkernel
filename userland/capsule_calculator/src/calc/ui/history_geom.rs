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

use super::metrics::{PANE_PAD, RAIL_W, READOUT_H};
use crate::calc::history::CAP;

pub const ROW_H: i32 = 44;
pub const ROW_GAP: i32 = 6;
pub const PAD: i32 = 14;

pub fn top() -> i32 {
    PANE_PAD + READOUT_H + PANE_PAD
}

pub fn pane(win_w: i32, win_h: i32) -> (i32, i32, i32, i32) {
    let y = top();
    (RAIL_W + PANE_PAD, y, win_w - RAIL_W - PANE_PAD * 2, win_h - y - PANE_PAD)
}

pub fn row(win_w: i32, i: usize) -> (i32, i32, i32, i32) {
    (
        RAIL_W + PANE_PAD + PAD,
        top() + PAD + i as i32 * (ROW_H + ROW_GAP),
        win_w - RAIL_W - PANE_PAD * 2 - PAD * 2,
        ROW_H,
    )
}

pub fn capacity(win_h: i32) -> usize {
    let avail = win_h - PANE_PAD - top() - PAD * 2;
    if avail < ROW_H {
        return 0;
    }
    (((avail + ROW_GAP) / (ROW_H + ROW_GAP)) as usize).min(CAP)
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

pub fn at(win_w: i32, win_h: i32, x: i32, y: i32) -> Option<usize> {
    for i in 0..capacity(win_h) {
        let (rx, ry, rw, rh) = row(win_w, i);
        if rw > 0 && x >= rx && x < rx + rw && y >= ry && y < ry + rh {
            return Some(i);
        }
    }
    None
}
