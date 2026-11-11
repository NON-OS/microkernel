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
use crate::calc::convert::CATEGORIES;

const CHIP_H: i32 = 36;
const GAP: i32 = 8;
const HEAD_H: i32 = 26;
const ROW_H: i32 = 32;
const ROW_GAP: i32 = 6;
const SWAP_W: i32 = 92;
const SWAP_H: i32 = 36;
const RESULT_H: i32 = 72;
/// The air between the chip strip and the unit columns, in logical pixels.
const CHIPS_TO_COLS: i32 = 14;
pub const CHIPS: usize = CATEGORIES.len();

/// The height of a unit column's heading, in drawing pixels.
pub fn head_h() -> i32 {
    px(HEAD_H)
}

pub fn pane(win_w: i32) -> (i32, i32) {
    let m = metrics();
    (m.rail_w + m.pane_pad, win_w - m.rail_w - m.pane_pad * 2)
}

pub fn chip_top() -> i32 {
    let m = metrics();
    m.pane_pad + m.readout_h + m.pane_pad
}

pub fn chip(win_w: i32, i: usize) -> Rect {
    let (x, pw) = pane(win_w);
    let cw = (pw - px(GAP) * (CHIPS as i32 - 1)) / CHIPS as i32;
    rect(x + i as i32 * (cw + px(GAP)), chip_top(), cw, px(CHIP_H))
}

pub fn col_top() -> i32 {
    chip_top() + px(CHIP_H) + px(CHIPS_TO_COLS)
}

pub fn col_w(win_w: i32) -> i32 {
    let (_, pw) = pane(win_w);
    (pw - px(SWAP_W) - px(GAP) * 2) / 2
}

pub fn row(win_w: i32, from: bool, i: usize) -> Rect {
    let (x, _) = pane(win_w);
    let (w, row_h) = (col_w(win_w), px(ROW_H));
    let x = if from { x } else { x + w + px(SWAP_W) + px(GAP) * 2 };
    let y = col_top() + head_h() + i as i32 * (row_h + px(ROW_GAP));
    rect(x, y, w, row_h)
}

pub fn swap(win_w: i32) -> Rect {
    let (x, _) = pane(win_w);
    let y = col_top() + head_h() + (px(ROW_H) + px(ROW_GAP)) * 2;
    rect(x + col_w(win_w) + px(GAP), y, px(SWAP_W), px(SWAP_H))
}

pub fn result(win_w: i32, win_h: i32) -> Rect {
    let (x, pw) = pane(win_w);
    rect(x, win_h - metrics().pane_pad - px(RESULT_H), pw, px(RESULT_H))
}
