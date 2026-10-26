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

//! The drop-down terminal: F12 brings the window down under the menubar, the
//! full width of the screen and 45% of its height, and F12 again, or Esc on an
//! empty line with nothing else open, sends it back where it was. The rect is
//! a request the window manager may clip (app_skeleton runner/rect_ask.rs).

use nonos_app_skeleton::{WorkRect, KEY_ESC, KEY_F12};

/// Where the dropped window stands, in thousandths of the work area.
pub const DROP: WorkRect = WorkRect { x: 0, y: 0, w: 1000, h: 450 };

/// The rect the window asks for: the drop while dropped, else nothing.
pub fn rect(dropped: bool) -> Option<WorkRect> {
    dropped.then_some(DROP)
}

/// Whether the window is dropped after key `code` with modifiers `flags`,
/// or None when the key is not the drop-down's. `idle` is an empty line with
/// no search, find, picker or program holding the keys: only then is Esc the
/// drop-down's, never taken from what it would close.
pub fn key(dropped: bool, code: u32, flags: u16, idle: bool) -> Option<bool> {
    match code {
        KEY_F12 if flags == 0 => Some(!dropped),
        KEY_ESC if dropped && idle && flags == 0 => Some(false),
        _ => None,
    }
}
