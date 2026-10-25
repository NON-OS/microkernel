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

//! Where things go inside the column: two halves of a span, a screen's
//! content across the column, and the lowest row content may reach.

use super::Layout;

/// Two columns `gap` apart in the span from `x`, `w` wide: the left and the
/// right, each as its left edge and width.
pub fn halves(x: u32, w: u32, gap: u32) -> ((u32, u32), (u32, u32)) {
    let left = w.saturating_sub(gap) / 2;
    let right = w.saturating_sub(gap + left);
    ((x, left), (x + left + gap, right))
}

/// Where a screen's content goes across the column: its left edge and width,
/// `side` in from each edge of the column.
pub fn content(l: &Layout, side: u32) -> (u32, u32) {
    (l.x + side, l.column.saturating_sub(2 * side))
}

/// The lowest row content may reach in a window `height` tall whose foot is
/// `foot` tall, `room` clear of it, so the last line never meets the foot or
/// the window's edge.
pub fn content_bottom(height: u32, foot: u32, room: u32) -> u32 {
    height.saturating_sub(foot + room)
}
