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

//! The right-click menu: a press while it is open picks a row or dismisses
//! it, and a right-press on the desktop opens it for the icon or empty space.

use super::open_item::open_item;
use crate::render::layout::{bottom_dock_rect, menubar_height};
use crate::render::{desktop_icons, desktop_menu};
use crate::server::desktop;
use crate::state::Context;

/// While the menu is open, the next press picks an item or dismisses it.
///
/// A press off its rows only takes the menu away, which repaints where it
/// and its shadow were (render/desktop_menu/frame.rs reach); a row picked
/// does what it names and repaints whole.
pub(super) fn pick(ctx: &mut Context, px: u32, py: u32) {
    let row = desktop_menu::hit(ctx, px, py);
    let was = desktop_menu::reach(ctx).filter(|_| row.is_none());
    match (ctx.desk_ui.menu_target, row) {
        // Empty-desktop menu.
        (None, Some(0)) => desktop::create_entry(ctx, false),
        (None, Some(1)) => desktop::create_entry(ctx, true),
        // Per-item menu: Open / Rename / Delete.
        (Some(idx), Some(0)) => open_item(ctx, idx),
        (Some(idx), Some(1)) => desktop::start_rename(ctx, idx),
        (Some(idx), Some(2)) => desktop::ask_delete(ctx, idx),
        _ => {}
    }
    ctx.desk_ui.desktop_menu = None;
    ctx.desk_ui.menu_hover = None;
    ctx.desk_ui.menu_target = None;
    match was {
        Some(r) => super::super::repaint::repaint_part(ctx, r),
        None => super::super::repaint::repaint(ctx),
    }
}

/// Open the menu at the press, between the menu bar and the dock. With no
/// drop-down to close, only the menu and its shadow are repainted.
pub(super) fn open(ctx: &mut Context, px: u32, py: u32) {
    let dock_top = bottom_dock_rect(ctx.width, ctx.height).y.saturating_sub(18);
    if py > menubar_height() && py < dock_top {
        let alone = ctx.menubar.open.is_none();
        ctx.desk_ui.menu_target = desktop_icons::hit(ctx, px, py);
        ctx.menubar.open = None;
        ctx.menubar.hover = None;
        ctx.desk_ui.desktop_menu = Some((px, py));
        ctx.desk_ui.menu_hover = None;
        match desktop_menu::reach(ctx).filter(|_| alone) {
            Some(r) => super::super::repaint::repaint_part(ctx, r),
            None => super::super::repaint::repaint(ctx),
        }
    }
}
