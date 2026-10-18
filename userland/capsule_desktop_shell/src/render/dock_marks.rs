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

//! What the dock draws besides its tiles: the running dot, the rule before
//! the Launchpad slot, and the Launchpad button.

use super::layout::{bottom_dock_rect, dock_divider_w, launchpad_slot_x, taskbar_entry_w};
use super::surface::surface;
use super::{palette, ui_font};
use crate::state::Context;

pub(super) fn icon_size() -> u32 {
    taskbar_entry_w()
}

// The rule between the app run and the Launchpad slot.
pub(super) fn draw_divider(ctx: &Context, box_top: u32, box_h: u32) {
    let slot_x = launchpad_slot_x(bottom_dock_rect(ctx.width, ctx.height));
    let sc = ui_font::scale();
    let h = box_h.saturating_sub(ui_font::px(12));
    let x = slot_x.saturating_sub(dock_divider_w() / 2);
    surface(ctx).fill_rect(x, box_top + (box_h - h) / 2, sc, h, palette::LINE);
}

/// The launchpad button: the brand mark, at the end of the dock.
///
/// It was a three by three grid of dots, which is the generic "more" glyph
/// every launcher uses and says nothing about whose machine this is. The mark
/// is already rasterised from the brand SVG and drawn elsewhere in this shell;
/// the one place a person looks for the system itself was the one place it did
/// not appear.
pub(super) fn draw_launchpad_button(ctx: &Context, box_top: u32, box_h: u32) {
    let slot_x = launchpad_slot_x(bottom_dock_rect(ctx.width, ctx.height));
    // Sized to the tile the way an app icon is, so the row reads evenly rather
    // than ending on something larger or smaller than its neighbours.
    let size = icon_size();
    let x = slot_x + (taskbar_entry_w().saturating_sub(size)) / 2;
    let y = box_top + (box_h.saturating_sub(size)) / 2;
    super::icons::draw_logo(ctx, x, y, size);
}

// The running indicator: a dot under the tile, accented when the app holds
// focus and dimmed when it is merely open.
pub(super) fn running_dot(ctx: &Context, cx: u32, active: bool) {
    let dock = bottom_dock_rect(ctx.width, ctx.height);
    let cy = dock.y + dock.height - ui_font::px(2);
    let argb = if active { palette::ACCENT } else { palette::ACCENT_DIM };
    surface(ctx).circle(cx, cy, ui_font::px(2), argb);
}
