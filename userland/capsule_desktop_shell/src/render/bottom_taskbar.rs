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

use alloc::vec::Vec;

use super::dock_view::{self, Tile};
use super::layout::{
    bottom_dock_rect, dock_box_inset, dock_divider_w, dock_pad, dock_stride, launchpad_slot_x,
    taskbar_entry_w,
};
use super::surface::surface;
use super::ui_font;
use crate::state::{Context, DOCK_APPS, LAUNCHER_APPS, TASKBAR_NO_ACTIVE};
use nonos_toolkit::widgets::Rect;

/* An app's mark at the share of its tile the Launchpad's badges give it. */
const GLYPH_LOGICAL: u32 = 21;

fn icon_size() -> u32 {
    taskbar_entry_w()
}

/* The app tiles' boxes, which the paint, the light and the click all use. */
fn tile_boxes(width: u32, height: u32) -> Vec<Rect> {
    let dock = bottom_dock_rect(width, height);
    let (left, top) = (dock.x + dock_pad(), dock.y + dock_box_inset());
    let h = dock.h - 2 * dock_box_inset();
    dock_view::row(left, top, taskbar_entry_w(), h, dock_stride(), DOCK_APPS)
}

pub fn dock_tile_hit(width: u32, height: u32, x: u32, y: u32) -> Option<usize> {
    dock_view::hit(&tile_boxes(width, height), x as i32, y as i32)
}

pub fn dock_lit(ctx: &Context) -> (Option<usize>, Option<usize>) {
    dock_view::lit(&tile_boxes(ctx.width, ctx.height), &ctx.tile_pointer)
}

fn draw_divider(ctx: &Context, box_top: u32, box_h: u32) {
    let slot_x = launchpad_slot_x(bottom_dock_rect(ctx.width, ctx.height));
    let sc = ui_font::scale();
    let h = box_h.saturating_sub(ui_font::px(12));
    let x = slot_x.saturating_sub(dock_divider_w() / 2);
    surface(ctx).blend_rect(x, box_top + (box_h - h) / 2, sc, h, ctx.palette.line);
}

fn draw_launchpad_button(ctx: &Context, box_top: u32, box_h: u32) {
    let slot_x = launchpad_slot_x(bottom_dock_rect(ctx.width, ctx.height));
    // Sized to the tile the way an app icon is, so the row reads evenly rather
    // than ending on something larger or smaller than its neighbours.
    let size = icon_size();
    let x = slot_x + (taskbar_entry_w().saturating_sub(size)) / 2;
    let y = box_top + (box_h.saturating_sub(size)) / 2;
    super::icons::draw_logo(ctx, x, y, size);
}

fn running_dot(ctx: &Context, cx: u32, active: bool) {
    let dock = bottom_dock_rect(ctx.width, ctx.height);
    let cy = dock.y + dock.h - ui_font::px(2);
    let argb = if active { ctx.palette.accent } else { ctx.palette.accent_dim };
    surface(ctx).circle(cx, cy, ui_font::px(2), argb);
}

fn tile(ctx: &Context, index: usize) -> Tile {
    let t = &ctx.taskbar;
    let app = &LAUNCHER_APPS[index];
    Tile {
        icon: super::icons::icon_id(app.icon),
        open: t.open[index],
        active: t.active != TASKBAR_NO_ACTIVE && t.active as usize == index,
        pulsing: t.pulse_until_ms[index] > 0,
        off: crate::apps_off::is_off(app.service),
    }
}

pub fn paint_bottom_taskbar(ctx: &Context) {
    let boxes = tile_boxes(ctx.width, ctx.height);
    let tiles: Vec<Tile> = (0..boxes.len()).map(|i| tile(ctx, i)).collect();
    dock_view::draw(&mut surface(ctx), &tiles, &boxes, GLYPH_LOGICAL, &ctx.tile_pointer);
    for (t, at) in tiles.iter().zip(&boxes) {
        if t.open || t.active || t.pulsing {
            running_dot(ctx, at.x + at.w / 2, t.active);
        }
        if t.off {
            super::off_tile::marker(ctx, at.x, at.y, icon_size());
        }
    }
    let (box_top, box_h) = boxes.first().map_or((0, 0), |b| (b.y, b.h));
    draw_divider(ctx, box_top, box_h);
    draw_launchpad_button(ctx, box_top, box_h);
}
