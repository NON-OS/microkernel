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

//! The Browse listing as plain numbers, taken from the state at the size last
//! painted, so the list's painter, the click and the hover read one layout and
//! a proof can build one without a store behind it.

/// What the listing lays out from: the window's size, whether the grid is on
/// show, how many entries there are and the first one drawn, and the row and
/// grid metrics `layout_measure::measure` stashed.
#[derive(Clone, Copy, Debug)]
pub struct Pane {
    pub win_w: u32,
    pub win_h: u32,
    pub grid: bool,
    pub len: usize,
    pub scroll: usize,
    pub row_top: u32,
    pub row_h: u32,
    pub view_rows: usize,
    pub grid_cols: u32,
}
