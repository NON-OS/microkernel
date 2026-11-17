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

use super::metrics::metrics;
use super::rect::rect;

pub fn row_y(i: usize) -> i32 {
    let m = metrics();
    m.nav_top + (i as i32) * (m.nav_h + m.nav_gap)
}

pub fn row_x() -> i32 {
    let m = metrics();
    m.rail_x + m.rail_pad_x
}

pub fn row_w() -> i32 {
    let m = metrics();
    m.rail_card_w - m.rail_pad_x * 2
}

/// The box the rail draws the `i`th mode's entry in.
pub fn row(i: usize) -> Rect {
    rect(row_x(), row_y(i), row_w(), metrics().nav_h)
}
