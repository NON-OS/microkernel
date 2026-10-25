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

//! A finished report onto the kernel ring, in order: the position, the
//! motion, the wheel, then the buttons.

use nonos_libc::{
    INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL,
    INPUT_KIND_WHEEL,
};

use super::pointer::Pointer;
use crate::post::send;

/// A post the kernel ring refuses is dropped, as
/// every input driver drops it: the next report carries the position anew.
pub fn post_pointer(p: &Pointer) {
    if p.abs_moved {
        send(INPUT_KIND_POINTER_ABS, 0, 0, p.abs, (0, 0));
    }
    if p.rel != (0, 0) {
        send(INPUT_KIND_POINTER_REL, 0, 0, (0, 0), p.rel);
    }
    if p.wheel != 0 {
        send(INPUT_KIND_WHEEL, 0, 0, (0, 0), (0, p.wheel));
    }
    for &(n, down) in &p.buttons[..p.n_buttons] {
        let kind = if down { INPUT_KIND_BUTTON_DOWN } else { INPUT_KIND_BUTTON_UP };
        send(kind, 0, n, (0, 0), (0, 0));
    }
}
