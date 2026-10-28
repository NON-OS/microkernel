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

use super::super::present::frame::now::to_content;
use super::super::present::frame::pointer as frame;
use super::super::present::scale_math::to_logical;
use super::axis::wheel;
use super::button::button;
use super::key::key;
use super::send::motion;

/// F11, which takes a guest window full screen and back (toplevel.rs).
const KEY_F11: u32 = nonos_app_skeleton::KEY_F1 + 10;

pub fn deliver(guest: &mut Guest, event: &InputEvent) {
    super::super::clip::focus::input(guest, event.kind);
    match event.kind {
        InputKind::KeyDown if event.code == KEY_F11 && super::super::toplevel::toggle(guest) => {}
        InputKind::KeyUp
            if event.code == KEY_F11 && super::super::toplevel::has_toplevel(guest) => {}
        InputKind::KeyDown => key(guest, event.code, event.flags, 1),
        InputKind::KeyUp => key(guest, event.code, event.flags, 0),
        // The NØNOS frame takes what lands on it (present/frame/pointer.rs).
        InputKind::PointerAbs => {
            let on = frame::motion(guest, event.x, event.y);
            if let Some((x, y)) = on.map(|p| logical(guest, p)) {
                motion(guest, x, y);
            }
        }
        InputKind::ButtonDown if !frame::button(guest, event.code, true) => {
            button(guest, event.code, 1)
        }
        InputKind::ButtonUp if !frame::button(guest, event.code, false) => {
            button(guest, event.code, 0)
        }
        InputKind::Wheel => {
            let on = to_content(&guest.scene, event.x, event.y);
            if let Some((x, y)) = on.map(|p| logical(guest, p)) {
                wheel(guest, event.delta_x, event.delta_y, x, y);
            }
        }
        _ => {}
    }
}

/// A point on the window's content as one on its surface, in the logical
/// pixels the client draws in.
fn logical(guest: &Guest, (x, y): (i32, i32)) -> (i32, i32) {
    let e = guest.scene.frame.ratio;
    (to_logical(x, e), to_logical(y, e))
}
