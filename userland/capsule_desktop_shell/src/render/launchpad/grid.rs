// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Grid geometry for the Launchpad: a centred, row-major grid holding every
//! desktop app, then every installed tool, then every capsule-store app.

use crate::render::ui_font;

const TILE_LOGICAL: u32 = 72;
const CELL_W_LOGICAL: u32 = 120;
const CELL_H_LOGICAL: u32 = 108;
const TITLE_Y_LOGICAL: u32 = 56;
const SEARCH_Y_LOGICAL: u32 = 96;
const SEARCH_W_LOGICAL: u32 = 360;
const SEARCH_H_LOGICAL: u32 = 36;
const GRID_TOP_LOGICAL: u32 = 168;
const DOTS_BAND_LOGICAL: u32 = 44;
const ROWS_MAX: u32 = 5;

pub(super) fn tile() -> u32 {
    ui_font::px(TILE_LOGICAL)
}

pub(super) fn cell_w() -> u32 {
    ui_font::px(CELL_W_LOGICAL)
}

pub(super) fn cell_h() -> u32 {
    ui_font::px(CELL_H_LOGICAL)
}

pub(super) fn title_y() -> u32 {
    ui_font::px(TITLE_Y_LOGICAL)
}

pub(super) fn search_y() -> u32 {
    ui_font::px(SEARCH_Y_LOGICAL)
}

pub(super) fn search_w() -> u32 {
    ui_font::px(SEARCH_W_LOGICAL)
}

pub(super) fn search_h() -> u32 {
    ui_font::px(SEARCH_H_LOGICAL)
}

pub(super) fn grid_top() -> u32 {
    ui_font::px(GRID_TOP_LOGICAL)
}

pub(super) fn dots_band() -> u32 {
    ui_font::px(DOTS_BAND_LOGICAL)
}

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
