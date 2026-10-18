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

//! A key while a menu is open: Esc closes the menu bar's drop-down or the
//! desktop's right-click menu, and any other key is swallowed.

use crate::server::repaint::repaint;
use crate::state::dialog_keys::KEY_ESC;
use crate::state::Context;

/// Esc closes the menu bar's drop-down or the desktop's right-click menu.
/// Any other key is swallowed while one is open: the menu holds the keys,
/// and a letter typed into it must not reach the window under it.
pub(super) fn menu_key(ctx: &mut Context, code: u32) -> bool {
    let open = ctx.menubar.open.is_some() || ctx.desk_ui.desktop_menu.is_some();
    if !open {
        return false;
    }
    if code == KEY_ESC {
        ctx.menubar.open = None;
        ctx.menubar.hover = None;
        ctx.desk_ui.desktop_menu = None;
        ctx.desk_ui.menu_hover = None;
        ctx.desk_ui.menu_target = None;
        repaint(ctx);
    }
    true
}
