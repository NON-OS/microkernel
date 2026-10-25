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

//! A position onto the kernel input ring. The buttons and the wheel come
//! from the PS/2 mouse, which the host keeps feeding them; a click lands at
//! the position posted here.

use nonos_libc::{mk_input_event_post, InputEvent, INPUT_KIND_POINTER_ABS};

/// False when the ring refused it; the next event carries the position anew.
pub fn post_position(at: (i32, i32)) -> bool {
    let ev = InputEvent {
        kind: INPUT_KIND_POINTER_ABS,
        flags: 0,
        code: 0,
        x: at.0,
        y: at.1,
        delta_x: 0,
        delta_y: 0,
        timestamp_ns: 0,
    };
    mk_input_event_post(&ev) >= 0
}
