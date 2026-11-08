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

//! A shared `Menu` over the editor, lit by the editor's `Pointer`. The row it
//! lights comes from the menu's own hit test, so a row lights where a click
//! on it lands. The menu bar, the ribbon and the explorer all open through
//! here.

use alloc::string::String;
use alloc::vec::Vec;
use nonos_toolkit::widgets::{Item, Menu, Pointer, Rect, Surface};

/// The whole window, `width` by `height`, that a menu is kept inside.
pub fn bounds(width: u32, height: u32) -> Rect {
    Rect { x: 0, y: 0, w: width, h: height }
}

/// The rect the menu of `items` is drawn at: right under `anchor`, kept
/// inside `bounds`.
pub fn place(items: &[Item], anchor: Rect, bounds: Rect) -> Rect {
    Menu::new(items).place(anchor, bounds)
}

/// The row of `items` under the point, when the menu is drawn at `at`.
pub fn hit(items: &[Item], at: Rect, x: i32, y: i32) -> Option<usize> {
    Menu::new(items).hit(at, x, y)
}

/// The row the pointer hovers on the menu of `items` drawn at `at`.
pub fn lit(items: &[Item], at: Rect, pointer: &Pointer) -> Option<usize> {
    let (x, y) = pointer.position()?;
    let i = hit(items, at, x, y)?;
    pointer.state(|x, y| hit(items, at, x, y) == Some(i)).hovered.then_some(i)
}

/// Draws the menu of `items` at `at` with the row `pointer` lights.
pub fn draw(fb: &mut impl Surface, items: &[Item], at: Rect, pointer: &Pointer) {
    Menu::new(items).hovered(lit(items, at, pointer)).draw(fb, at);
}

/// One row per value in `labels`, the one naming `current`, the value in
/// effect, ticked. Every row has a tick slot, so the rows sit the same
/// whichever value is in effect.
pub fn ticked<'a>(labels: &'a [String], current: &str) -> Vec<Item<'a>> {
    labels.iter().map(|l| Item::new(l).checked(l == current)).collect()
}
