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

use crate::settings::section::{Section, SECTIONS};

use super::metrics::metrics;

pub fn row_y(index: usize) -> u32 {
    let m = metrics();
    m.nav_top + index as u32 * (m.nav_h + m.nav_gap)
}

pub fn row_x() -> u32 {
    metrics().nav_pad_x / 2
}

pub fn row_w() -> u32 {
    let m = metrics();
    m.sidebar_w - m.nav_pad_x
}

/// Which nav entry covers `y`, in sidebar-local coordinates. The painter walks
/// the same `row_y`, so a click cannot land on a different entry than the one
/// drawn under the pointer.
pub fn at(x: i32, y: i32) -> Option<Section> {
    let m = metrics();
    if x < 0 || x >= m.sidebar_w as i32 || y < m.nav_top as i32 {
        return None;
    }
    let offset = (y - m.nav_top as i32) as u32;
    let index = (offset / (m.nav_h + m.nav_gap)) as usize;
    if offset % (m.nav_h + m.nav_gap) >= m.nav_h {
        return None;
    }
    SECTIONS.get(index).copied()
}
