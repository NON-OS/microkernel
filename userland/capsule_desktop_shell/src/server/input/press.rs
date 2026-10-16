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

//! A press on the desktop, once nothing drawn over it took the press: the
//! menu bar, the right-click menu, the magnifier and brand, an icon, the
//! hidden dock's edge, and otherwise the window under it.

use crate::protocol::read_u32;
use crate::server::handlers::menubar_click;
use crate::state::Context;
use nonos_libc::INPUT_KIND_BUTTON_DOWN;

pub(super) fn press(ctx: &mut Context, buf: &[u8], kind: u16, x: i32, y: i32) -> bool {
    let (px, py) = (x as u32, y as u32);
    if super::prompts::consumed(ctx, px, py) {
        return true;
    }
    // A click on a menu-bar title, or anywhere while a drop-down is open,
    // belongs to the menu bar.
    if menubar_click::click(ctx, px, py) {
        super::super::repaint::repaint(ctx);
        return true;
    }
    // While the right-click menu is open, the next click either picks an item
    // or dismisses it. Handle that before anything else consumes the click.
    if ctx.desk_ui.desktop_menu.is_some() {
        super::menu::pick(ctx, px, py);
        return true;
    }
    // Right-click (button code 2) on the desktop opens a menu: New Folder /
    // New File on empty space, or Open / Rename / Delete on an icon.
    if kind == INPUT_KIND_BUTTON_DOWN && read_u32(buf, 12) == Some(2) {
        super::menu::open(ctx, px, py);
        return true;
    }
    super::desk::desk(ctx, x, y)
}
