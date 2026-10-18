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

//! The pointer moving: an icon drag following it, a menu-bar drop-down or
//! the right-click menu tracking it, and otherwise the dock's reveal.

use super::hover::hover_reveal;
use crate::render::desktop_menu;
use crate::server::handlers::menubar_click;
use crate::state::Context;

/// Pointer travel, in pixels, before a press on an icon becomes a drag.
const DRAG_THRESHOLD: u32 = 6;

pub(super) fn motion(ctx: &mut Context, x: i32, y: i32) -> bool {
    // An icon drag in progress follows the pointer once it clears the
    // threshold, and takes priority over menu hover and dock reveal.
    if ctx.desk_ui.drag_from.is_some() {
        let (nx, ny) = (x as u32, y as u32);
        if ctx.desk_ui.drag_moved {
            ctx.desk_ui.drag_x = nx;
            ctx.desk_ui.drag_y = ny;
            super::super::repaint::repaint(ctx);
        } else if nx.abs_diff(ctx.desk_ui.drag_x) > DRAG_THRESHOLD
            || ny.abs_diff(ctx.desk_ui.drag_y) > DRAG_THRESHOLD
        {
            ctx.desk_ui.drag_moved = true;
            ctx.desk_ui.drag_x = nx;
            ctx.desk_ui.drag_y = ny;
            super::super::repaint::repaint(ctx);
        }
        return true;
    }
    // A menu-bar drop-down tracks the pointer across its own titles and
    // rows for as long as it is open.
    if ctx.menubar.open.is_some() {
        if menubar_click::motion(ctx, x as u32, y as u32) {
            super::super::repaint::repaint(ctx);
        }
        return true;
    }
    // With the menu open, track which row the pointer is over so it lights
    // up; otherwise fall through to the dock reveal behaviour.
    if ctx.desk_ui.desktop_menu.is_some() {
        let hovered = desktop_menu::hit(ctx, x as u32, y as u32);
        if hovered != ctx.desk_ui.menu_hover {
            ctx.desk_ui.menu_hover = hovered;
            super::super::repaint::repaint(ctx);
        }
        return true;
    }
    hover_reveal(ctx, y as u32);
    true
}
