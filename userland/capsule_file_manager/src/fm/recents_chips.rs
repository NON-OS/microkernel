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

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use super::chip::{chip_gap, chip_h, chip_w};
use super::filetype::Kind;
use super::layout::{metrics, px};
use super::recents_filter::kind_label;
use super::screen_row::label_adv;

/// One laid-out filter pill. `kind` is `None` for the All chip. The label is
/// carried rather than rebuilt, so the hit-test measures the exact string the
/// painter drew.
pub struct KindChip {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub kind: Option<Kind>,
    pub label: String,
}

/// Top of a filter chip row, directly under the surface's section label.
pub fn chips_top() -> u32 {
    let m = metrics();
    m.header_h + px(16) + label_adv()
}

/// The one wrapped chip row both filtering surfaces lay out: an All chip
/// carrying the whole count, then one chip per kind the data actually holds.
/// A pill moves to the next line when the measured one would overrun.
pub fn kind_chips(counts: &[(Kind, u32)], win_w: u32) -> Vec<KindChip> {
    let m = metrics();
    let left = m.sidebar_w + m.pad_x;
    let right = left + win_w.saturating_sub(m.sidebar_w + m.pad_x * 2);
    let total: u32 = counts.iter().map(|(_, n)| n).sum();
    let mut items: Vec<(Option<Kind>, String)> = Vec::new();
    items.push((None, alloc::format!("All {total}")));
    for (kind, n) in counts {
        items.push((Some(*kind), alloc::format!("{} {}", kind_label(*kind), n)));
    }
    let (mut x, mut y) = (left, chips_top());
    let mut out = Vec::new();
    for (kind, label) in items {
        let w = chip_w(label.as_str());
        if x > left && x + w > right {
            x = left;
            y += chip_h() + chip_gap();
        }
        out.push(KindChip { x, y, w, kind, label });
        x += w + chip_gap();
    }
    out
}

/// The y just past the chip row, where the filtered listing starts.
pub fn chips_bottom(chips: &[KindChip]) -> u32 {
    let m = metrics();
    chips.last().map(|c| c.y).unwrap_or_else(chips_top) + chip_h() + m.section_gap
}
