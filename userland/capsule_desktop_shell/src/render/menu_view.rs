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

//! A shared `Menu` on a shell layer, lit by the shell's `Pointer`. The rows it
//! lights come from the menu's own hit test, so a row lights where a click
//! on it lands.

use nonos_toolkit::widgets::{Menu, Pointer, Rect, Surface};

/// The row the pointer hovers and the row held down on the menu drawn at
/// `at`. A row is held only while a press that began on it stays on it.
pub fn lit(menu: &Menu, at: Rect, pointer: &Pointer) -> (Option<usize>, Option<usize>) {
    let row = |x: i32, y: i32| menu.hit(at, x, y);
    let Some(i) = pointer.position().and_then(|(x, y)| row(x, y)) else {
        return (None, None);
    };
    let s = pointer.state(|x, y| row(x, y) == Some(i));
    (s.hovered.then_some(i), s.pressed.then_some(i))
}

/// Draws `menu` at `at` with the rows `pointer` lights.
pub fn draw(fb: &mut impl Surface, menu: Menu, at: Rect, pointer: &Pointer) {
    let (hovered, pressed) = lit(&menu, at, pointer);
    menu.hovered(hovered).pressed(pressed).draw(fb, at);
}

/// The whole layer, `width` by `height`, that a menu is kept inside.
pub fn bounds(width: u32, height: u32) -> Rect {
    Rect { x: 0, y: 0, w: width, h: height }
}
