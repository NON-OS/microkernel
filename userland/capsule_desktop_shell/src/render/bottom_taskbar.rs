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

//! The dock: a tile for each app, then the Launchpad slot.

use super::dock_marks::{draw_divider, draw_launchpad_button, icon_size, running_dot};
use super::layout::{bottom_dock_rect, dock_box_inset, dock_gap, dock_pad, taskbar_entry_w, Rect};
use super::palette;
use crate::state::{Context, DOCK_APPS, LAUNCHER_APPS, TASKBAR_NO_ACTIVE};

const TILE_RADIUS_LOGICAL: u32 = 10;

pub fn paint_bottom_taskbar(ctx: &Context) {
    let dock = bottom_dock_rect(ctx.width, ctx.height);
    let box_top = dock.y + dock_box_inset();
    let box_h = dock.height - 2 * dock_box_inset();
    let mut x = dock.x + dock_pad();
    for (index, app) in LAUNCHER_APPS.iter().enumerate().take(DOCK_APPS) {
        let open = ctx.taskbar.open[index];
        let active =
            ctx.taskbar.active != TASKBAR_NO_ACTIVE && ctx.taskbar.active as usize == index;
        let pulsing = ctx.taskbar.pulse_until_ms[index] > 0;
        let bg = if active {
            palette::TILE_ACTIVE
        } else if pulsing {
            palette::TILE_PULSE
        } else if open {
            palette::TILE_OPEN
        } else {
            super::off_tile::idle_fill(app.service)
        };
        let tile = Rect { x, y: box_top, width: taskbar_entry_w(), height: box_h };
        let edge = if active || pulsing { palette::LINE_HARD } else { palette::LINE_SOFT };
        super::panel::panel(ctx, tile, TILE_RADIUS_LOGICAL, bg, edge);
        if open || active || pulsing {
            running_dot(ctx, x + taskbar_entry_w() / 2, active);
        }
        super::off_tile::dock_glyph(ctx, x, box_top, app, icon_size());
        x += taskbar_entry_w() + dock_gap();
    }
    draw_divider(ctx, box_top, box_h);
    draw_launchpad_button(ctx, box_top, box_h);
}
