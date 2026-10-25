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

//! Where the menu bar and the dock are on the screen, and all the dock
//! paints there.

use super::super::ui_font;
use super::dock::{bottom_dock_bottom_inset, bottom_dock_height, bottom_dock_width};

const MENUBAR_H_LOGICAL: u32 = 46;

pub fn menubar_height() -> u32 {
    ui_font::px(MENUBAR_H_LOGICAL)
}

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn menubar_rect(display_width: u32) -> Rect {
    Rect { x: 0, y: 0, width: display_width, height: menubar_height() }
}

pub fn bottom_dock_rect(display_width: u32, display_height: u32) -> Rect {
    let w = core::cmp::min(bottom_dock_width(), display_width);
    let h = core::cmp::min(bottom_dock_height(), display_height);
    let x = display_width.saturating_sub(w) / 2;
    let y = display_height.saturating_sub(h + bottom_dock_bottom_inset());
    Rect { x, y, width: w, height: h }
}

/// Everything the dock puts on the screen: its panel and the shadow drawn
/// round it (render/shadow_reach.rs; `px(3)`, render/panel.rs
/// shadow_spread, and a pixel for rounding), down to the bottom edge. A
/// repaint of the dock commits this whole rect, so a dock hidden over a
/// full-screen window leaves none of its panel or its shadow behind. Its top
/// is also where the pointer leaves the dock's area.
pub fn dock_area_rect(display_width: u32, display_height: u32) -> Rect {
    let d = bottom_dock_rect(display_width, display_height);
    let spread = ui_font::px(3) + 1;
    let (x, y, width, _) = super::super::shadow_reach::with_shadow(
        (d.x, d.y, d.width, d.height),
        spread,
        display_width,
        display_height,
    );
    Rect { x, y, width, height: display_height.saturating_sub(y) }
}
