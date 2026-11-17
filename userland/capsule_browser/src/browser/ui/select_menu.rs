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

//! An open select's list as a shared `Menu`: where it opens, the row a point
//! is on and how it is drawn, all from one menu, so they never disagree.

use alloc::vec::Vec;

use nonos_toolkit::widgets::{Item, Menu, Rect, Surface};

use crate::browser::event::select_list::{Row, SelectList, MAX_ROWS};

/// Where the list opens for its select drawn at `select`, under it or over
/// it, inside `bounds`, both in page-area drawing pixels.
pub fn place(list: &SelectList, select: Rect, bounds: Rect) -> Rect {
    menu(list, &items(list)).place(select, bounds)
}

/// Sizes a list opening by its select drawn at `select`, inside `bounds`:
/// as wide as its widest row or the select, holding the rows that fit.
pub fn open_at(list: &mut SelectList, select: Rect, bounds: Rect) {
    let w = Menu::new(&items(list)).size().0;
    (list.widest, list.fits) = (w.max(select.w), MAX_ROWS);
    list.fits = menu(list, &items(list)).shown(place(list, select, bounds));
    list.step(0);
}

/// The row under (`x`, `y`) on the list drawn at `at`, if it can be chosen.
pub fn row_at(list: &SelectList, at: Rect, x: i32, y: i32) -> Option<usize> {
    menu(list, &items(list)).hit(at, x, y).map(|i| i + list.first)
}

/// Draws the list at `at`, the highlighted row lit, a pressed row held.
pub fn draw(list: &SelectList, fb: &mut impl Surface, at: Rect) {
    let (items, p) = (items(list), list.pointer);
    let m = menu(list, &items);
    let under = p.position().and_then(|(x, y)| m.hit(at, x, y));
    let held = under.filter(|&i| p.state(|x, y| m.hit(at, x, y) == Some(i)).pressed);
    m.pressed(held).draw(fb, at);
}

fn menu<'a>(list: &SelectList, items: &'a [Item<'a>]) -> Menu<'a> {
    let end = (list.first + list.shown()).min(items.len());
    let shown = &items[list.first.min(end)..end];
    Menu::new(shown).width(list.widest).hovered(list.hi.checked_sub(list.first))
}

fn items(list: &SelectList) -> Vec<Item<'_>> {
    let grouped = list.rows.iter().any(|r| r.heading);
    list.rows.iter().map(|r| item(r, grouped)).collect()
}

fn item(r: &Row, grouped: bool) -> Item<'_> {
    let it = Item::new(&r.label).checked(r.selected);
    match (r.heading, grouped, r.disabled) {
        (true, ..) => Item::heading(&r.label),
        (_, true, true) => it.indent().disabled(),
        (_, true, false) => it.indent(),
        (_, false, true) => it.disabled(),
        _ => it,
    }
}
