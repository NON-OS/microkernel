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

use super::metrics::{PANE_PAD, RAIL_W, READOUT_H};
use super::rect::rect;
use crate::calc::convert::CATEGORIES;

pub const CHIP_H: i32 = 36;
pub const GAP: i32 = 8;
pub const HEAD_H: i32 = 26;
pub const ROW_H: i32 = 32;
pub const ROW_GAP: i32 = 6;
pub const SWAP_W: i32 = 92;
pub const SWAP_H: i32 = 36;
pub const RESULT_H: i32 = 72;
pub const CHIPS: usize = CATEGORIES.len();

pub fn pane(win_w: i32) -> (i32, i32) {
    (RAIL_W + PANE_PAD, win_w - RAIL_W - PANE_PAD * 2)
}

pub fn chip_top() -> i32 {
    PANE_PAD + READOUT_H + PANE_PAD
}

pub fn chip(win_w: i32, i: usize) -> Rect {
    let (px, pw) = pane(win_w);
    let cw = (pw - GAP * (CHIPS as i32 - 1)) / CHIPS as i32;
    rect(px + i as i32 * (cw + GAP), chip_top(), cw, CHIP_H)
}

pub fn col_top() -> i32 {
    chip_top() + CHIP_H + 14
}

pub fn col_w(win_w: i32) -> i32 {
    let (_, pw) = pane(win_w);
    (pw - SWAP_W - GAP * 2) / 2
}

pub fn row(win_w: i32, from: bool, i: usize) -> Rect {
    let (px, _) = pane(win_w);
    let w = col_w(win_w);
    let x = if from { px } else { px + w + SWAP_W + GAP * 2 };
    let y = col_top() + HEAD_H + i as i32 * (ROW_H + ROW_GAP);
    rect(x, y, w, ROW_H)
}

pub fn swap(win_w: i32) -> Rect {
    let (px, _) = pane(win_w);
    let y = col_top() + HEAD_H + (ROW_H + ROW_GAP) * 2;
    rect(px + col_w(win_w) + GAP, y, SWAP_W, SWAP_H)
}

pub fn result(win_w: i32, win_h: i32) -> Rect {
    let (px, pw) = pane(win_w);
    rect(px, win_h - PANE_PAD - RESULT_H, pw, RESULT_H)
}
