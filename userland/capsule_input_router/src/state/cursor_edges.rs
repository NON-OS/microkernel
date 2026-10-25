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

//! The cursor's edges: the display the router places the pointer on. They
//! are known only once the compositor has given its size, by a notice on every
//! change (NIRS op 0x0006) or by answering DISPLAY_INFO. Until then no pointer
//! event is placed: on a made-up 1024x768, a click on a 1920x1080 display
//! lands at x * 1024 / 1920. An absolute position is held instead, and
//! placed on the real edges the moment they arrive.

use nonos_libc::{InputEvent, INPUT_KIND_POINTER_ABS, INPUT_KIND_TOUCH};

use super::{CursorState, ABS_RANGE_MAX};

impl CursorState {
    /// The display is `width` by `height` from now on: the first size sets the
    /// edges, a later one moves them at once. False for a size with a zero
    /// side, which changes nothing.
    pub fn adopt(&mut self, width: u32, height: u32) -> bool {
        if width == 0 || height == 0 {
            return false;
        }
        if self.configured {
            self.resize(width, height);
        } else {
            self.configure(width, height);
        }
        if let Some((x, y)) = self.held.take() {
            (self.x, self.y) = (place(x, self.max_x), place(y, self.max_y));
        }
        true
    }

    /// Keep where an absolute device points while the edges are unknown.
    pub fn hold(&mut self, ev: &InputEvent) {
        if ev.kind == INPUT_KIND_POINTER_ABS || ev.kind == INPUT_KIND_TOUCH {
            self.held = Some((ev.x, ev.y));
        }
    }
}

/// An absolute coordinate, 0 to INPUT_ABS_MAX, on an axis whose last pixel is `max`.
pub(super) fn place(v: i32, max: i32) -> i32 {
    (v as i64 * max as i64 / ABS_RANGE_MAX) as i32
}
