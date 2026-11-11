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

//! Metrics for the whole editor shell: an activity bar and file-explorer
//! sidebar on the left, a tab strip along the top, the code pane, and a status
//! bar at the bottom. The code pane's origin is computed from the chrome around
//! it, and the real monospace cell width is measured from the font each paint;
//! `GLYPH_ADVANCE` is only the pre-measure fallback.

use nonos_toolkit::design::scale::scale;

// Window chrome and shell panels, in logical pixels.
const TITLEBAR_H: u32 = 26;
const TABBAR_H: u32 = 32;
// The formatting ribbon between the tab strip and the pane. Chrome text clamps
// to 17px, so the row is tall enough for a 27px control cell plus its padding.
const RIBBON_H: u32 = 40;
const FOOTER_H: u32 = 26;
const ACTIVITY_W: u32 = 46;
const SIDEBAR_W: u32 = 220;

// Code pane internals, relative to the pane origin.
const GUTTER_W: u32 = 54;
const TEXT_PAD: u32 = 12;
const PAD_X: u32 = 12;
const PAD_TOP: u32 = 8;
const ROW_H: u32 = 24;

// Font sizes in logical px. Chrome stays fixed; only the body zooms.
const CHROME_PX: f32 = 14.0;

// Zoom bounds. Scale 2 is the default and reproduces the original 15px body.
pub(super) const MIN_SCALE: u32 = 1;
pub(super) const MAX_SCALE: u32 = 6;

/// The menu bar's height, in drawing pixels.
pub(super) fn titlebar_h() -> u32 {
    scale().px(TITLEBAR_H)
}

/// The tab strip's height, in drawing pixels.
pub(super) fn tabbar_h() -> u32 {
    scale().px(TABBAR_H)
}

/// The formatting ribbon's height, in drawing pixels.
pub(super) fn ribbon_h() -> u32 {
    scale().px(RIBBON_H)
}

/// The status bar's height, in drawing pixels.
pub(super) fn footer_h() -> u32 {
    scale().px(FOOTER_H)
}

/// The activity bar's width, in drawing pixels.
pub(super) fn activity_w() -> u32 {
    scale().px(ACTIVITY_W)
}

/// The open sidebar's width, in drawing pixels.
pub(super) fn sidebar_w() -> u32 {
    scale().px(SIDEBAR_W)
}

/// The line-number gutter's width, in drawing pixels.
pub(super) fn gutter_w() -> u32 {
    scale().px(GUTTER_W)
}

/// The gap above the code pane's first row, in drawing pixels.
pub(super) fn pad_top() -> u32 {
    scale().px(PAD_TOP)
}

/// One explorer row, in drawing pixels.
pub(super) fn row_h() -> u32 {
    scale().px(ROW_H)
}

/// The chrome's text size, in drawing pixels.
pub(super) fn chrome_px() -> f32 {
    scale().font(CHROME_PX)
}

// Body font size for a zoom level in logical px: 11 + 2*scale, matching the
// terminal's zoom curve (scale 2 = 15px). Line height and gutter size track
// it so a zoom change moves everything together.
pub(super) fn zoom_px(scale: u32) -> u32 {
    11 + scale.clamp(MIN_SCALE, MAX_SCALE) * 2
}

pub(super) fn body_px(zoom: u32) -> f32 {
    scale().font(zoom_px(zoom) as f32)
}

pub(super) fn gutter_px(zoom: u32) -> f32 {
    scale().font((zoom_px(zoom) as f32 - 2.0).max(9.0))
}

pub(super) fn line_height(zoom: u32) -> u32 {
    scale().px(zoom_px(zoom) + 7)
}

/// Fallback cell width before the first measured advance is known.
pub(super) const GLYPH_ADVANCE: u32 = 9;

// The first text column's x, relative to the code pane's left edge.
pub(super) fn text_left(pane_x: u32) -> u32 {
    pane_x + gutter_w() + scale().px(TEXT_PAD)
}

pub(super) fn wrap_cols(pane_w: u32, advance: u32) -> u32 {
    let inset = gutter_w() + scale().px(TEXT_PAD) + scale().px(PAD_X);
    pane_w.saturating_sub(inset).saturating_div(advance.max(1)).clamp(16, 400)
}
