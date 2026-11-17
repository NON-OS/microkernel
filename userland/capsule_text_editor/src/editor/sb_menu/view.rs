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

//! The context menu as the shared `Menu`: its rows, where it opens by the
//! press that brought it up, the action under a click, and its paint. The
//! painter and the hit test both build it here, so a click lands on the row
//! it was drawn under.

use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Item, Pointer, Rect};

use super::{MenuAction, SbMenu};
use crate::editor::menu_view::{draw, hit, lit, place};

/// One row per action, in the menu's order. Delete removes the file at
/// once, so it draws as a danger row.
fn items(menu: &SbMenu) -> Vec<Item<'static>> {
    let item = |&a: &MenuAction| match a {
        MenuAction::Delete => Item::new(a.label()).danger(),
        _ => Item::new(a.label()),
    };
    menu.items.iter().map(item).collect()
}

/// Where the menu is drawn: under the press that opened it, kept inside
/// `bounds`.
fn at(menu: &SbMenu, rows: &[Item], bounds: Rect) -> Rect {
    place(rows, Rect { x: menu.x, y: menu.y, w: 0, h: 0 }, bounds)
}

/// The action under a click, or None when the click falls outside the menu.
pub(in crate::editor) fn menu_hit(
    menu: &SbMenu,
    bounds: Rect,
    x: i32,
    y: i32,
) -> Option<MenuAction> {
    let rows = items(menu);
    hit(&rows, at(menu, &rows, bounds), x, y).and_then(|i| menu.items.get(i).copied())
}

/// Draws the context menu over whatever is beneath it.
pub(in crate::editor) fn paint_menu(
    fb: &mut PaintBuffer,
    menu: &SbMenu,
    bounds: Rect,
    pointer: &Pointer,
) {
    let rows = items(menu);
    draw(fb, &rows, at(menu, &rows, bounds), pointer);
}

/// The row the pointer lights.
pub(in crate::editor) fn menu_lit(menu: &SbMenu, bounds: Rect, pointer: &Pointer) -> Option<usize> {
    let rows = items(menu);
    lit(&rows, at(menu, &rows, bounds), pointer)
}
