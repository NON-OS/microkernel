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
use super::button::{button, press_at};
use super::key::key;
use super::send::motion;

/// F11, which takes a guest window full screen and back (toplevel.rs).
const KEY_F11: u32 = nonos_app_skeleton::KEY_F1 + 10;

/// `event`, for the guest's window manager window `window` when the router
/// names one: a pointer event goes to that window, a key to the one with the
/// keyboard (window_front/).
pub fn deliver(guest: &mut Guest, event: &InputEvent, window: Option<u32>) {
    // What this changes is not held behind a quiet stream's poll.
    crate::linux::net::input_delivered();
    // A popup in a window of its own: the point on its toplevel's surface.
    let on_popup = window.and_then(|id| super::popup_point::point(guest, id, (event.x, event.y)));
    let _ = match (event.kind, window) {
        (InputKind::KeyDown | InputKind::KeyUp, _) => {
            super::super::window_front::select_focused(guest)
        }
        _ if on_popup.is_some() => true,
        (_, Some(id)) => super::super::window_front::select_window(guest, id),
        (_, None) => true,
    };
    super::super::clip::focus::input(guest, event.kind);
    if super::drag::taken(guest, event) || super::dismiss::taken(guest, event) {
        return;
    }
    if let Some(at) = on_popup {
        return super::popup_point::deliver(guest, event, at);
    }
    match event.kind {
        InputKind::KeyDown if event.code == KEY_F11 && super::super::toplevel::toggle(guest) => {}
        InputKind::KeyUp
            if event.code == KEY_F11 && super::super::toplevel::has_toplevel(guest) => {}
        InputKind::KeyDown => key(guest, event.code, event.flags, 1),
        InputKind::KeyUp => key(guest, event.code, event.flags, 0),
        // The NØNOS frame takes what lands on it (present/frame/pointer.rs).
        // Onto the frame, the pointer leaves the client's surface.
        InputKind::PointerAbs => {
            match frame::motion(guest, event.x, event.y).map(|p| logical(guest, p)) {
                Some((x, y)) => motion(guest, x, y),
                None => super::off::pointer_off(guest),
            }
        }
        InputKind::ButtonDown if !frame::button(guest, event.code, true) => {
            match to_content(&guest.scene, event.x, event.y).map(|p| logical(guest, p)) {
                Some(at) => press_at(guest, event.code, at),
                None => button(guest, event.code, 1),
            }
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
/// pixels the client draws in, from the corner its window geometry names.
pub(super) fn logical(guest: &Guest, (x, y): (i32, i32)) -> (i32, i32) {
    let (e, (ox, oy)) = (guest.scene.win.frame.ratio, guest.scene.win.origin);
    (to_logical(x, e).saturating_add(ox), to_logical(y, e).saturating_add(oy))
}
