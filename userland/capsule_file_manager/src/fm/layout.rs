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

use nonos_toolkit::design::scale::scale;

// Curated list geometry. The row count is derived from the live window height
// each frame (see `measure`) so the list fills the window when resized or
// maximized instead of stranding a fixed 7 rows in the top-left corner.
const HEADER_H: u32 = 78;
const FOOTER_H: u32 = 38;
const ROW_H: u32 = 44;
const PAD_X: u32 = 24;
// Left PLACES sidebar; the file list and chrome start at its right edge.
const SIDEBAR_W: u32 = 236;
const SIDE_ROW_H: u32 = 34;
const SIDE_FIRST_Y: u32 = 92;

// Row/scroll geometry consumed by paint, click hit-testing, and scroll
// clamping. Kept as the first-row origin and default fallbacks.
const FIRST_ROW_Y: u32 = HEADER_H + 6;
pub const LIST_VISIBLE: usize = 8;

// Icon-grid metrics: each cell holds one large icon and its label below it.
const GRID_TOP: u32 = HEADER_H + 12;
const GRID_CELL_W: u32 = 124;
const GRID_CELL_H: u32 = 112;

// The vertical gap between the titled sections every screen stacks its
// content into.
const SECTION_GAP: u32 = 26;

// Browse reserves a fixed right-hand strip for the info panel. The width is
// stated once here and every Browse consumer -- the list, the grid's column
// count, and the panel itself -- derives its own geometry from these three
// functions, so the listing and the panel can never overlap.
const INFO_W: u32 = 268;

// The window shell's corner radius in logical pixels.
const R_SHELL: u32 = 14;

/// The window's frame at the display scale in force, in drawing pixels.
pub struct Metrics {
    pub header_h: u32,
    pub footer_h: u32,
    pub row_h: u32,
    pub pad_x: u32,
    pub sidebar_w: u32,
    pub side_row_h: u32,
    pub side_first_y: u32,
    pub first_row_y: u32,
    pub grid_top: u32,
    pub grid_cell_w: u32,
    pub grid_cell_h: u32,
    pub section_gap: u32,
    pub info_w: u32,
    pub r_shell: u32,
}

/// The frame at the display scale in force: each logical size above read once
/// through `scale()`, so no caller scales one again.
pub fn metrics() -> Metrics {
    Metrics {
        header_h: px(HEADER_H),
        footer_h: px(FOOTER_H),
        row_h: px(ROW_H),
        pad_x: px(PAD_X),
        sidebar_w: px(SIDEBAR_W),
        side_row_h: px(SIDE_ROW_H),
        side_first_y: px(SIDE_FIRST_Y),
        first_row_y: px(FIRST_ROW_Y),
        grid_top: px(GRID_TOP),
        grid_cell_w: px(GRID_CELL_W),
        grid_cell_h: px(GRID_CELL_H),
        section_gap: px(SECTION_GAP),
        info_w: px(INFO_W),
        r_shell: px(R_SHELL),
    }
}

/// `logical` pixels at the display scale in force, in drawing pixels.
pub fn px(logical: u32) -> u32 {
    scale().px(logical)
}

/// A logical type size at the display scale in force, in drawing pixels.
pub fn font(logical: f32) -> f32 {
    scale().font(logical)
}

/// The scale's hairline, in drawing pixels.
pub fn hair() -> u32 {
    scale().hair()
}

/// Left edge of the Browse listing.
pub fn content_x() -> u32 {
    let m = metrics();
    m.sidebar_w + m.pad_x
}

/// Width left to the Browse listing once the info panel strip is reserved.
pub fn content_w(win_w: u32) -> u32 {
    let m = metrics();
    win_w.saturating_sub(m.sidebar_w + m.pad_x * 3 + m.info_w).max(m.grid_cell_w)
}

/// Left edge of the info panel strip.
pub fn info_x(win_w: u32) -> u32 {
    let m = metrics();
    win_w.saturating_sub(m.info_w + m.pad_x)
}
