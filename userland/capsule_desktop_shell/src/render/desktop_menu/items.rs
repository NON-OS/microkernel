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

//! The right-click menu's rows and where it opens. The painter and the hit
//! test both take the menu and its rect from here.

use super::rows::{EMPTY_ITEMS, ITEM_ITEMS};
use crate::render::menu_view::bounds;
use crate::state::Context;
use nonos_toolkit::widgets::{Menu, Rect};

/// The open menu, chosen by whether it targets an item, and the rect it is
/// drawn at: under the press that opened it, kept on the screen.
pub(super) fn placed(ctx: &Context) -> Option<(Menu<'static>, Rect)> {
    let (x, y) = ctx.desktop_menu?;
    let m = Menu::new(if ctx.menu_target.is_some() { &ITEM_ITEMS } else { &EMPTY_ITEMS });
    let at = m.place(Rect { x, y, w: 0, h: 0 }, bounds(ctx.width, ctx.height));
    Some((m, at))
}
