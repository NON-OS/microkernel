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

//! One input event onto the kernel ring.

use nonos_libc::{mk_input_event_post, InputEvent};

pub fn send(kind: u16, flags: u16, code: u32, at: (i32, i32), delta: (i32, i32)) -> bool {
    let ev = InputEvent {
        kind,
        flags,
        code,
        x: at.0,
        y: at.1,
        delta_x: delta.0,
        delta_y: delta.1,
        timestamp_ns: 0,
    };
    mk_input_event_post(&ev) >= 0
}
