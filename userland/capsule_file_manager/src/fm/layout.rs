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

// Curated list geometry. The row count is derived from the live window height
// each frame (see `measure`) so the list fills the window when resized or
// maximized instead of stranding a fixed 7 rows in the top-left corner.
pub const HEADER_H: u32 = 78;
pub const FOOTER_H: u32 = 38;
pub const ROW_H: u32 = 44;
pub const PAD_X: u32 = 24;
// Left PLACES sidebar; the file list and chrome start at CONTENT_X.
pub const SIDEBAR_W: u32 = 236;
pub const CONTENT_X: u32 = SIDEBAR_W;
pub const SIDE_ROW_H: u32 = 34;
pub const SIDE_FIRST_Y: u32 = 92;

// Row/scroll geometry consumed by paint, click hit-testing, and scroll
// clamping. Kept as the first-row origin and default fallbacks.
pub const FIRST_ROW_Y: u32 = HEADER_H + 6;
pub const LIST_VISIBLE: usize = 8;

// Icon-grid metrics: each cell holds one large icon and its label below it.
pub const GRID_TOP: u32 = HEADER_H + 12;
pub const GRID_CELL_W: u32 = 124;
pub const GRID_CELL_H: u32 = 112;

// The vertical gap between the titled sections every screen stacks its
// content into.
pub const SECTION_GAP: u32 = 26;

// Browse reserves a fixed right-hand strip for the info panel. The width is
// stated once here and every Browse consumer -- the list, the grid's column
// count, and the panel itself -- derives its own geometry from these three
// functions, so the listing and the panel can never overlap.
pub const INFO_W: u32 = 268;

// The window shell's corner radius in logical pixels.
pub const R_SHELL: u32 = 14;

/// Left edge of the Browse listing.
pub fn content_x() -> u32 {
    CONTENT_X + PAD_X
}

/// Width left to the Browse listing once the info panel strip is reserved.
pub fn content_w(win_w: u32) -> u32 {
    win_w.saturating_sub(CONTENT_X + PAD_X * 3 + INFO_W).max(GRID_CELL_W)
}

/// Left edge of the info panel strip.
pub fn info_x(win_w: u32) -> u32 {
    win_w.saturating_sub(INFO_W + PAD_X)
}
