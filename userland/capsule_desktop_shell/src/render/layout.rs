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

use super::ui_font;
use crate::state::DOCK_APPS;

const TASKBAR_ENTRY_W_LOGICAL: u32 = 46;
const DOCK_GAP_LOGICAL: u32 = 7;
const DOCK_PAD_LOGICAL: u32 = 12;
const DOCK_BOX_INSET_LOGICAL: u32 = 9;
const DOCK_DIVIDER_LOGICAL: u32 = 11;
const MENUBAR_H_LOGICAL: u32 = 46;

pub fn menubar_height() -> u32 {
    ui_font::px(MENUBAR_H_LOGICAL)
}

/// One slot per desktop app, plus a trailing slot for the Launchpad button.
/// Summed from the same rounded parts the slots are placed with, so at a
/// fractional scale the last slot still ends inside the dock.
pub fn bottom_dock_width() -> u32 {
    let slots = DOCK_APPS as u32 + 1;
    slots * dock_stride() - dock_gap() + dock_divider_w() + 2 * dock_pad()
}

/// One app slot and the gap after it.
pub fn dock_stride() -> u32 {
    taskbar_entry_w() + dock_gap()
}

pub fn bottom_dock_height() -> u32 {
    ui_font::px(64)
}

pub fn bottom_dock_bottom_inset() -> u32 {
    ui_font::px(16)
}

/// Width of the rule that separates the app run from the Launchpad slot,
/// margins included.
pub fn dock_divider_w() -> u32 {
    ui_font::px(DOCK_DIVIDER_LOGICAL)
}

pub fn dock_gap() -> u32 {
    ui_font::px(DOCK_GAP_LOGICAL)
}

pub fn dock_pad() -> u32 {
    ui_font::px(DOCK_PAD_LOGICAL)
}

/// Vertical inset from the dock edge to the row of entry tiles.
pub fn dock_box_inset() -> u32 {
    ui_font::px(DOCK_BOX_INSET_LOGICAL)
}

pub fn taskbar_entry_w() -> u32 {
    ui_font::px(TASKBAR_ENTRY_W_LOGICAL)
}

/// Left edge of the Launchpad button: the slot just past the last app.
pub fn launchpad_slot_x(dock: Rect) -> u32 {
    dock.x + dock_pad() + DOCK_APPS as u32 * dock_stride() + dock_divider_w()
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
    let (x, y, width, _) = super::shadow_reach::with_shadow(
        (d.x, d.y, d.width, d.height),
        spread,
        display_width,
        display_height,
    );
    Rect { x, y, width, height: display_height.saturating_sub(y) }
}
