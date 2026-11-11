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

use super::bits_geom;
use super::metrics::{metrics, px};

const ROW_H: i32 = 24;
const PAD: i32 = 8;
const GAP_ABOVE: i32 = 10;
const KEYPAD_GAP: i32 = 12;

/// The panel's inset around its rows, in drawing pixels.
pub fn pad() -> i32 {
    px(PAD)
}

/// The gap between the panel and the keypad under it, in drawing pixels.
pub fn keypad_gap() -> i32 {
    px(KEYPAD_GAP)
}

fn panel_h() -> i32 {
    px(ROW_H) * 4 + pad() * 2
}

pub fn top() -> i32 {
    bits_geom::bottom() + px(GAP_ABOVE)
}

pub fn bottom() -> i32 {
    top() + panel_h()
}

pub fn origin() -> (i32, i32) {
    let m = metrics();
    (m.rail_w + m.pane_pad, top())
}

pub fn size(win_w: i32) -> (i32, i32) {
    let m = metrics();
    (win_w - m.rail_w - m.pane_pad * 2, panel_h())
}

pub fn row_y(i: usize) -> i32 {
    top() + pad() + (i as i32) * px(ROW_H)
}
