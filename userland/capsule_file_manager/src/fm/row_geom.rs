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

use alloc::vec::Vec;

use super::state::State;

/// One drawn list line: the entry it shows and the y it was placed at.
pub struct RowSlot {
    pub index: usize,
    pub y: u32,
}

/// The one layout pass for the detail list, in the shape `sidebar_rows` uses:
/// `paint_rows` draws exactly these slots and `row_at` tests against them, so a
/// click can never land on a row that was not drawn.
pub fn row_slots(state: &State) -> Vec<RowSlot> {
    (state.scroll..state.entries.len())
        .take(state.view_rows)
        .enumerate()
        .map(|(vis, index)| RowSlot { index, y: state.row_top + vis as u32 * state.row_h })
        .collect()
}

/// Where the row for `index` was drawn, or `None` when it is scrolled out.
pub fn row_y(state: &State, index: usize) -> Option<u32> {
    row_slots(state).into_iter().find(|s| s.index == index).map(|s| s.y)
}

/// Which entry the point `y` lands on; `None` above the first row, below the
/// last drawn one, or past the end of the listing.
pub fn row_at(state: &State, y: u32) -> Option<usize> {
    let h = state.row_h.max(1);
    row_slots(state).into_iter().find(|s| y >= s.y && y < s.y + h).map(|s| s.index)
}
