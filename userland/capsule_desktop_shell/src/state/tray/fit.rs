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

//! How many tray labels the menu bar has room for. The labels sit between the
//! menu titles and the status cluster; when they do not all fit, the first
//! ones that do are shown and a "+N" mark of `more_w` says how many are not,
//! so an item is never dropped from the bar without a word.

/// The number of leading labels, of the given widths, that fit in `room`
/// with `gap` between neighbours, leaving space for the "+N" mark whenever
/// any label is left out.
pub fn fit(widths: &[u32], room: u32, gap: u32, more_w: u32) -> usize {
    if span(widths, gap) <= room {
        return widths.len();
    }
    let mut shown = widths.len();
    while shown > 0 {
        shown -= 1;
        let mark = if shown == 0 { more_w } else { gap + more_w };
        if span(&widths[..shown], gap).saturating_add(mark) <= room {
            return shown;
        }
    }
    0
}

/// The width of `widths` laid out in a row with `gap` between neighbours.
pub fn span(widths: &[u32], gap: u32) -> u32 {
    let gaps = gap.saturating_mul(widths.len().saturating_sub(1) as u32);
    widths.iter().fold(gaps, |sum, w| sum.saturating_add(*w))
}
