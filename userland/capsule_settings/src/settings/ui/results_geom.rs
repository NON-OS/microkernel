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

use super::metrics::metrics;

pub fn card_y() -> u32 {
    let m = metrics();
    m.pane_pad_top + m.head_h
}

pub fn card_h(n: usize) -> u32 {
    let m = metrics();
    m.card_head_h + m.row_h * n.max(1) as u32
}

pub fn row_y(i: usize) -> u32 {
    let m = metrics();
    card_y() + m.card_head_h + m.row_h * i as u32
}

pub fn content_h(n: usize) -> u32 {
    card_y() + card_h(n) + metrics().card_gap
}

pub fn max_scroll(n: usize, view_h: u32) -> u32 {
    content_h(n).saturating_sub(view_h)
}

pub fn index_at(y: i32, scroll: u32, n: usize) -> Option<usize> {
    let first = row_y(0) as i32;
    let abs = y + scroll as i32;
    if abs < first {
        return None;
    }
    let i = ((abs - first) / metrics().row_h as i32) as usize;
    if i < n {
        Some(i)
    } else {
        None
    }
}
