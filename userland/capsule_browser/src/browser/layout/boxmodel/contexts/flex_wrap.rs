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

use super::flex_item::FlexItem;

/* The end of the line that starts at item `a`: every item when the box
 * does not wrap, else as many as fit by their hypothetical sizes, and
 * always at least one. */
pub(in super::super) fn line_end(
    items: &[FlexItem],
    a: usize,
    w: i32,
    gap: i32,
    wrap: bool,
) -> usize {
    if !wrap {
        return items.len();
    }
    let mut used = items[a].outer();
    let mut b = a + 1;
    while b < items.len() && used.saturating_add(gap).saturating_add(items[b].outer()) <= w {
        used = used.saturating_add(gap).saturating_add(items[b].outer());
        b += 1;
    }
    b
}

/* How many auto margins on the main axis a line's items have. */
pub(in super::super) fn auto_margins(line: &[FlexItem]) -> i32 {
    let s = line.iter().map(|it| &it.node.style);
    s.map(|s| s.margin_left_auto as i32 + s.margin_right_auto as i32).sum()
}
