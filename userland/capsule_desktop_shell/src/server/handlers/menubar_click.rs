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

//! Menu-bar dispatch: a title click toggles its drop-down, pointer motion
//! switches titles or highlights a row while one is open.

use super::menubar_action::activate;
use crate::render::menubar_menu;
use crate::state::Context;
use nonos_toolkit::widgets::Pointer;

pub fn click(ctx: &mut Context, px: u32, py: u32) -> bool {
    if let Some(title) = menubar_menu::title_hit(ctx, px, py) {
        let same = ctx.menubar.open == Some(title);
        ctx.menubar.open = if same { None } else { Some(title) };
        ctx.menubar.pointer = Pointer::new();
        ctx.desktop_menu = None;
        ctx.menu_target = None;
        return true;
    }
    let Some(title) = ctx.menubar.open else {
        return false;
    };
    let row = menubar_menu::row_hit(ctx, px, py);
    ctx.menubar.open = None;
    ctx.menubar.pointer = Pointer::new();
    if let Some(row) = row {
        activate(ctx, title, row);
    }
    true
}

pub fn motion(ctx: &mut Context, px: u32, py: u32) -> bool {
    if ctx.menubar.open.is_none() {
        return false;
    }
    let (open, lit) = (ctx.menubar.open, menubar_menu::lit(ctx));
    if let Some(title) = menubar_menu::title_hit(ctx, px, py) {
        ctx.menubar.open = Some(title);
    }
    ctx.menubar.pointer.moved(px as i32, py as i32);
    ctx.menubar.open != open || menubar_menu::lit(ctx) != lit
}
