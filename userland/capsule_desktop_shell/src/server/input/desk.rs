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

//! A press on the desktop itself: the magnifier and brand on the menu bar,
//! an icon (the start of a drag), the hidden dock's edge, and otherwise the
//! window under the press.

use crate::render::layout::bottom_dock_rect;
use crate::render::{desktop_icons, topbar};
use crate::server::desktop;
use crate::server::handlers::launcher_focus;
use crate::state::{reveal_taskbar, Context};

pub(super) fn desk(ctx: &mut Context, x: i32, y: i32) -> bool {
    let (px, py) = (x as u32, y as u32);
    // The magnifier on the menu bar opens the Launchpad and its search, as the
    // Spotlight request does.
    if topbar::search_hit(ctx, px, py) {
        crate::server::handlers::spotlight_toggle::toggle(ctx);
        return true;
    }
    // Clicking the brand on the menu bar brings up the app dock.
    // Over a full-screen window it shows the hidden dock for a moment (the
    // dock's sync after this batch draws it and opens its window).
    if topbar::brand_hit(px, py) {
        reveal_taskbar(&mut ctx.taskbar, crate::server::dock_clock::now());
        return true;
    }
    // A left-press on an icon begins a potential drag. If the pointer never
    // moves, release opens it; if it moves onto a folder, release moves it
    // there. Both resolve in drop_drag. The pointer is grabbed for the whole
    // gesture so the release reaches the shell even over another window.
    if let Some(idx) = desktop_icons::hit(ctx, px, py) {
        ctx.desk_ui.drag_from = Some(idx);
        ctx.desk_ui.drag_moved = false;
        ctx.desk_ui.drag_x = px;
        ctx.desk_ui.drag_y = py;
        desktop::grab_drag(ctx);
        return true;
    }
    if !ctx.taskbar.visible {
        let dock = bottom_dock_rect(ctx.width, ctx.height);
        if y as u32 >= dock.y.saturating_sub(18) {
            reveal_taskbar(&mut ctx.taskbar, crate::server::dock_clock::now());
        }
        return true;
    }
    launcher_focus::handle(ctx, x as u32, y as u32);
    true
}
