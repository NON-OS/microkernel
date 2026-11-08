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

//! Shared Settings geometry. The rail's rows and the pane's left edge come
//! from here, and every control is laid out in the screen's `Controls` from
//! these boxes as it is painted, so a press lands where the row was drawn.

use nonos_toolkit::ttf::line_height;
use nonos_toolkit::widgets::{NavItem, Rect};

pub(super) const RAIL_X: u32 = 46;
pub(super) const RAIL_W: u32 = 250;
pub(super) const RAIL_PAD: u32 = 10;
pub(super) const PANE_PAD: u32 = 26;

pub(super) const TITLE_PX: f32 = 24.0;

pub(super) const NAV_LABELS: [&str; 2] = ["Editing", "Appearance"];

pub(super) fn title_top() -> u32 {
    22
}

pub(super) fn head_top() -> u32 {
    26
}

pub(super) fn lh(px: f32) -> u32 {
    line_height(px).max(1) as u32
}

/// Rail row `i`, under the screen title, one navigation item tall.
pub(super) fn nav_row(i: usize) -> Rect {
    let h = NavItem::new("").size().1;
    let y = title_top() + lh(TITLE_PX) + 18 + h * i as u32;
    Rect { x: RAIL_X + RAIL_PAD, y, w: RAIL_W - RAIL_PAD * 2, h }
}

pub(super) fn pane_x() -> u32 {
    RAIL_X + RAIL_W
}
