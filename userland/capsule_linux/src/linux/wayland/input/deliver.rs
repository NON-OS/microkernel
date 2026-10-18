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

//! One input event from the router, as the Wayland events it means. F11 is
//! the window's own: full screen and back, when the guest has a toplevel.

use nonos_app_skeleton::input::{InputEvent, InputKind};

use crate::linux::guest::Guest;

use super::axis::wheel;
use super::button::button;
use super::key::key;
use super::send::motion;

/// F11, which takes a guest window full screen and back (toplevel.rs).
const KEY_F11: u32 = nonos_app_skeleton::KEY_F1 + 10;

pub fn deliver(guest: &mut Guest, event: &InputEvent) {
    match event.kind {
        InputKind::KeyDown if event.code == KEY_F11 && super::super::toplevel::toggle(guest) => {}
        InputKind::KeyUp
            if event.code == KEY_F11 && super::super::toplevel::has_toplevel(guest) => {}
        InputKind::KeyDown => key(guest, event.code, event.flags, 1),
        InputKind::KeyUp => key(guest, event.code, event.flags, 0),
        InputKind::PointerAbs => motion(guest, event.x, event.y),
        InputKind::ButtonDown => button(guest, event.code, 1),
        InputKind::ButtonUp => button(guest, event.code, 0),
        InputKind::Wheel => wheel(guest, event.delta_x, event.delta_y, event.x, event.y),
        _ => {}
    }
}
