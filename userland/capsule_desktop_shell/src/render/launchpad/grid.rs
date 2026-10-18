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
