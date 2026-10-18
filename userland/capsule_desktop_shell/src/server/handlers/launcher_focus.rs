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

use super::launcher_launch::launch_at;
use crate::render::layout::{
    bottom_dock_rect, dock_box_inset, dock_gap, dock_pad, launchpad_slot_x, taskbar_entry_w,
};
use crate::server::handlers::launchpad;
use crate::state::{Context, DOCK_APPS, LAUNCHER_APPS};

pub fn handle(ctx: &mut Context, x: u32, y: u32) {
    let bottom = bottom_dock_rect(ctx.width, ctx.height);
    let mut row_x = bottom.x + dock_pad();
    for (index, app) in LAUNCHER_APPS.iter().enumerate().take(DOCK_APPS) {
        if x >= row_x
            && x < row_x + taskbar_entry_w()
            && y >= bottom.y + dock_box_inset()
            && y < bottom.y + bottom.height - dock_box_inset()
        {
            launch_at(ctx, index, app);
            return;
        }
        row_x += taskbar_entry_w() + dock_gap();
    }
    // The trailing dock slot is the Launchpad button.
    let lp = launchpad_slot_x(bottom);
    if x >= lp
        && x < lp + taskbar_entry_w()
        && y >= bottom.y + dock_box_inset()
        && y < bottom.y + bottom.height - dock_box_inset()
    {
        launchpad::open(ctx);
    }
}
