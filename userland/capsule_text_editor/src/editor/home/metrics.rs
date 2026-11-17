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

//! Home screen constants and left-rail geometry. Every rect the painters draw
//! and the router hit-tests is derived here or in `metrics_pane`, so a click
//! can never land somewhere the paint pass did not put the thing it hits.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::ttf::line_height;
use nonos_toolkit::widgets::{NavItem, Rect};

use crate::editor::layout::activity_w;

const RAIL_W: u32 = 246;
const RAIL_PAD: u32 = 16;
const PANE_PAD: u32 = 26;
const BRAND_SIDE: u32 = 28;
const BRAND_Y: u32 = 18;
const NAV_GAP: u32 = 22;
const BODY: f32 = 17.0;
const SUBHEAD: f32 = 19.0;
const HEAD: f32 = 34.0;

/// The rail's width, in drawing pixels.
pub(super) fn rail_w() -> u32 {
    scale().px(RAIL_W)
}

/// The rail's side inset, in drawing pixels.
pub(super) fn rail_pad() -> u32 {
    scale().px(RAIL_PAD)
}

/// The pane's inset on every side, in drawing pixels.
pub(super) fn pane_pad() -> u32 {
    scale().px(PANE_PAD)
}

/// The brand chip's side, in drawing pixels.
pub(super) fn brand_side() -> u32 {
    scale().px(BRAND_SIDE)
}

/// The brand chip's top, in drawing pixels.
pub(super) fn brand_y() -> u32 {
    scale().px(BRAND_Y)
}

/// The body text size, in drawing pixels.
pub(super) fn body() -> f32 {
    scale().font(BODY)
}

/// The wordmark's text size, in drawing pixels.
pub(super) fn subhead() -> f32 {
    scale().font(SUBHEAD)
}

/// The pane title's text size, in drawing pixels.
pub(super) fn head() -> f32 {
    scale().font(HEAD)
}

pub(super) fn lh(px: f32) -> u32 {
    line_height(px).max(1) as u32
}

pub(super) fn rail_x() -> u32 {
    activity_w()
}

pub(super) fn pane_x() -> u32 {
    activity_w() + rail_w()
}

/// Rail row `i`, under the brand, one navigation item tall.
pub(super) fn nav_row(i: usize) -> Rect {
    let h = NavItem::new("").size().1;
    let y = brand_y() + brand_side() + scale().px(NAV_GAP) + h * i as u32;
    Rect { x: rail_x() + rail_pad(), y, w: rail_w() - rail_pad() * 2, h }
}
