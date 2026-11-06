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

use super::metrics::{KEY_GAP, PANE_PAD, RAIL_W, READOUT_H};
use super::radix_geom;
use super::rect::rect;
use crate::calc::buttons::{grid, Button};
use crate::calc::mode::Mode;

pub fn top(mode: Mode) -> i32 {
    match mode {
        Mode::Programmer => radix_geom::bottom() + radix_geom::KEYPAD_GAP,
        _ => PANE_PAD + READOUT_H + PANE_PAD,
    }
}

pub fn area(mode: Mode, win_w: i32, win_h: i32) -> (i32, i32, i32, i32) {
    let x = RAIL_W + PANE_PAD;
    let y = top(mode);
    (x, y, win_w - RAIL_W - PANE_PAD * 2, win_h - y - PANE_PAD)
}

pub fn cols(mode: Mode) -> i32 {
    let mut widest = 0;
    for row in grid(mode) {
        let total: i32 = row.iter().map(|btn| btn.span.max(1) as i32).sum();
        widest = widest.max(total);
    }
    widest
}

pub fn stride(mode: Mode, win_w: i32, win_h: i32) -> (i32, i32, i32, i32, i32, i32) {
    let (ax, ay, aw, ah) = area(mode, win_w, win_h);
    let nc = cols(mode).max(1);
    let nr = (grid(mode).len() as i32).max(1);
    let cw = (aw - KEY_GAP * (nc - 1)) / nc;
    let ch = (ah - KEY_GAP * (nr - 1)) / nr;
    (ax, ay, cw, ch, nc, nr)
}

pub fn cell(mode: Mode, win_w: i32, win_h: i32, row: usize, col: usize, span: u8) -> Rect {
    let (ax, ay, cw, ch, _, _) = stride(mode, win_w, win_h);
    let n = span.max(1) as i32;
    let x = ax + col as i32 * (cw + KEY_GAP);
    let y = ay + row as i32 * (ch + KEY_GAP);
    rect(x, y, cw * n + KEY_GAP * (n - 1), ch)
}

/// Each key of `mode`'s keypad, row by row: its row, its place in that row,
/// the key and the box it is drawn in. The painter, the click and the hover
/// all walk this.
pub fn keys(
    mode: Mode,
    win_w: i32,
    win_h: i32,
) -> impl Iterator<Item = (usize, usize, &'static Button, Rect)> {
    grid(mode).iter().enumerate().flat_map(move |(row, buttons)| {
        let mut col = 0usize;
        buttons.iter().enumerate().map(move |(idx, btn)| {
            let span = btn.span.max(1);
            let at = cell(mode, win_w, win_h, row, col, span);
            col += span as usize;
            (row, idx, btn, at)
        })
    })
}
