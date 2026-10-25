// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Where the Launchpad's cells fall: how many rows and columns the screen
//! holds, so how many tiles a page, and where each cell's corner is.

use super::grid::{cell_h, cell_w, dots_band, grid_top};

const ROWS_MAX: u32 = 5;

pub(super) fn rows(height: u32) -> u32 {
    let budget = height.saturating_sub(grid_top()).saturating_sub(dots_band());
    (budget / cell_h()).clamp(1, ROWS_MAX)
}

pub(super) fn per_page(width: u32, height: u32) -> usize {
    (cols(width) * rows(height)) as usize
}

/// Columns that fit the display, kept within a sensible range so the grid stays
/// centred rather than stretching edge to edge on a wide screen.
pub(super) fn cols(width: u32) -> u32 {
    (width / cell_w()).clamp(1, 8)
}

/// Top-left screen position of the nth cell.
pub(super) fn cell_origin(width: u32, index: usize) -> (u32, u32) {
    let c = cols(width);
    let i = index as u32;
    let grid_w = c * cell_w();
    let left = width.saturating_sub(grid_w) / 2;
    (left + (i % c) * cell_w(), grid_top() + (i / c) * cell_h())
}
