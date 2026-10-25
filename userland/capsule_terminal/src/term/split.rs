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

//! Two tabs side by side in one window: where each pane falls, which tab is
//! on which side, and what becomes of the split when tabs move or close. The
//! keys go to the focused pane, the active tab; the other is the split.

/// Pixels between the panes, the divider drawn in them.
pub const DIVIDER: u32 = 2;

/// The two panes of a window `w` wide, as (x, width): the left half and the
/// right, the divider between them.
pub fn halves(w: u32) -> ((u32, u32), (u32, u32)) {
    let left = w.saturating_sub(DIVIDER) / 2;
    let right_x = left + DIVIDER;
    ((0, left), (right_x, w.saturating_sub(right_x)))
}

/// Which tab is drawn on the left and which on the right: the lower number
/// on the left, so focus moving between them never swaps the sides.
pub fn sides(active: usize, split: usize) -> (usize, usize) {
    (active.min(split), active.max(split))
}

/// The split once the active tab moves from `old` to `new`: moving onto the
/// split's tab trades places with it, so both stay on screen.
pub fn retarget(split: Option<usize>, old: usize, new: usize) -> Option<usize> {
    match split {
        Some(s) if s == new => Some(old),
        other => other,
    }
}

/// The tab in front once the active one closes, leaving `left` tabs: the
/// other pane when the window was split, one lower when it came after the
/// closed tab; otherwise the same place, or the new last tab.
pub fn after_close(split: Option<usize>, active: usize, left: usize) -> usize {
    match split {
        Some(s) if s > active => s - 1,
        Some(s) => s,
        None => active.min(left.saturating_sub(1)),
    }
}

/// Which pane a pointer at `x` is over in a window `w` wide: false the left,
/// true the right.
pub fn right_of(x: i32, w: u32) -> bool {
    let ((_, left), _) = halves(w);
    x >= 0 && x as u32 >= left + DIVIDER / 2
}
