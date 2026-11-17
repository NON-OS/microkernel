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

//! A shared `Dialog` on a shell layer, lit by the shell's `Pointer`. The
//! button it lights comes from the dialog's own hit test, so a button lights
//! where a click on it lands.

use nonos_toolkit::widgets::{Dialog, Pointer, Rect, Surface};

/// The button the pointer hovers and the button held down on the dialog
/// drawn at `at`. A button is held only while a press that began on it
/// stays on it.
pub fn lit(dialog: &Dialog, at: Rect, pointer: &Pointer) -> (Option<usize>, Option<usize>) {
    let button = |x: i32, y: i32| dialog.hit(at, x, y);
    let Some(i) = pointer.position().and_then(|(x, y)| button(x, y)) else {
        return (None, None);
    };
    let s = pointer.state(|x, y| button(x, y) == Some(i));
    (s.hovered.then_some(i), s.pressed.then_some(i))
}

/// Draws `dialog` over all of `at` with the buttons `pointer` lights.
pub fn draw(fb: &mut impl Surface, dialog: Dialog, at: Rect, pointer: &Pointer) {
    let (hovered, pressed) = lit(&dialog, at, pointer);
    dialog.hovered(hovered).pressed(pressed).draw(fb, at);
}
