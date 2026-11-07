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
use crate::render::layout::Rect;
use crate::render::{desktop_icons, desktop_menu, menubar_menu};
use crate::server::handlers::menubar_click;
use crate::state::Context;

/// Pointer travel, in pixels, before a press on an icon becomes a drag.
const DRAG_THRESHOLD: u32 = 6;

pub(super) fn motion(ctx: &mut Context, x: i32, y: i32) -> bool {
    // An icon drag in progress follows the pointer once it clears the
    // threshold, and takes priority over menu hover and dock reveal.
    // Each step repaints where the icon was drawn and where it is now
    // (render/desktop_icons/ghost.rs), not the whole screen.
    if ctx.desk_ui.drag_from.is_some() {
        let (nx, ny) = (x as u32, y as u32);
        let moved = ctx.desk_ui.drag_moved
            || nx.abs_diff(ctx.desk_ui.drag_x) > DRAG_THRESHOLD
            || ny.abs_diff(ctx.desk_ui.drag_y) > DRAG_THRESHOLD;
        if moved {
            let was = desktop_icons::ghost_rect(ctx);
            ctx.desk_ui.drag_moved = true;
            ctx.desk_ui.drag_x = nx;
            ctx.desk_ui.drag_y = ny;
            match desktop_icons::ghost_rect(ctx) {
                Some(now) => super::super::repaint::repaint_part(ctx, around(was, now)),
                None => super::super::repaint::repaint(ctx),
            }
        }
        return true;
    }
    // A menu-bar drop-down tracks the pointer across its own titles and
    // rows for as long as it is open.
    // A row lit or put out in the same drop-down changes its panel only;
    // another drop-down opened changes the bar and both panels.
    // Another title's drop-down opened changes the bar and the two
    // drop-downs' places (repaint_dropdown).
    if ctx.menubar.open.is_some() {
        let (open, was) = (ctx.menubar.open, menubar_menu::reach(ctx));
        if menubar_click::motion(ctx, x as u32, y as u32) {
            match menubar_menu::frame(ctx).filter(|_| ctx.menubar.open == open) {
                Some(panel) => super::super::repaint::repaint_part(ctx, panel),
                None => super::super::repaint::repaint_dropdown(ctx, was),
            }
        }
        return true;
    }
    // With the menu open, track which row the pointer is over so it lights
    // up, which changes the menu's panel only; otherwise fall through to the
    // dock reveal behaviour.
    if ctx.desk_ui.desktop_menu.is_some() {
        let hovered = desktop_menu::hit(ctx, x as u32, y as u32);
        if hovered != ctx.desk_ui.menu_hover {
            ctx.desk_ui.menu_hover = hovered;
            match desktop_menu::frame(ctx) {
                Some(panel) => super::super::repaint::repaint_part(ctx, panel),
                None => super::super::repaint::repaint(ctx),
            }
        }
        return true;
    }
    hover_reveal(ctx, y as u32);
    true
}

/// The box round `now` and, when there was one, `was`.
fn around(was: Option<Rect>, now: Rect) -> Rect {
    let Some(w) = was else { return now };
    let (x, y) = (w.x.min(now.x), w.y.min(now.y));
    let right = (w.x.saturating_add(w.width)).max(now.x.saturating_add(now.width));
    let bottom = (w.y.saturating_add(w.height)).max(now.y.saturating_add(now.height));
    Rect { x, y, width: right - x, height: bottom - y }
}
