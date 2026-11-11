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
use crate::calc::prog::BITS;

pub const COLS: i32 = 16;
const GAP: i32 = 4;
const BIT_H: i32 = 26;
const LABEL_H: i32 = 24;
const ROW_GAP: i32 = 4;

/// A bit's key height, in drawing pixels.
pub fn bit_h() -> i32 {
    px(BIT_H)
}

/// The height of the bit number under a key, in drawing pixels.
pub fn label_h() -> i32 {
    px(LABEL_H)
}

fn row_stride() -> i32 {
    bit_h() + label_h() + px(ROW_GAP)
}

fn band_h() -> i32 {
    row_stride() * 2 - px(ROW_GAP)
}

pub fn top() -> i32 {
    let m = metrics();
    m.pane_pad + m.readout_h + m.pane_pad
}

pub fn bottom() -> i32 {
    top() + band_h()
}

pub fn band(win_w: i32) -> (i32, i32, i32, i32) {
    let m = metrics();
    (m.rail_w + m.pane_pad, top(), win_w - m.rail_w - m.pane_pad * 2, band_h())
}

pub fn cell_w(win_w: i32) -> i32 {
    let (_, _, aw, _) = band(win_w);
    (aw - px(GAP) * (COLS - 1)) / COLS
}

pub fn cell(win_w: i32, bit: u8) -> Rect {
    let (ax, ay, _, _) = band(win_w);
    let cw = cell_w(win_w);
    let idx = (BITS as i32 - 1) - bit as i32;
    let (row, col) = (idx / COLS, idx % COLS);
    rect(ax + col * (cw + px(GAP)), ay + row * row_stride(), cw, bit_h())
}
