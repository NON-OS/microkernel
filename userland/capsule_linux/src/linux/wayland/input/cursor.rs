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

//! The pointer's shape over the window. The compositor draws the cursor
//! itself (cursor_code.rs): a shape asked for through wp_cursor_shape_v1 is
//! the nearest it draws, and a cursor image of the client's own
//! (wl_pointer.set_cursor) is drawn as the arrow, so a resize arrow asked for
//! before does not stay. Either is taken only with the serial of the
//! pointer's latest enter while it is on the client's surface, as both
//! protocols say; the pointer leaving puts the arrow back.

use nonos_app_skeleton::clients::compositor::cursor_shape;
use nonos_app_skeleton::discover::lookup_port;

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::cursor_code::{code_of, ARROW};

/// set_shape. A shape past those version 1 names is drawn as the arrow.
pub fn shape(guest: &mut Guest, args: &mut Args<'_>) {
    let (Some(serial), Some(shape)) = (args.u32(), args.u32()) else { return };
    if current(guest, serial) {
        set(guest, code_of(shape));
    }
}

/// wl_pointer.set_cursor: the client's own image, drawn as the arrow.
pub fn image(guest: &mut Guest, args: &mut Args<'_>) {
    if args.u32().is_some_and(|serial| current(guest, serial)) {
        set(guest, ARROW);
    }
}

/// The pointer left the client's surface.
pub fn arrow(guest: &mut Guest) {
    set(guest, ARROW);
}

/// The pointer on the NØNOS frame: the shape a press there acts with
/// (present/frame/resize.rs `shape`).
pub fn frame(guest: &mut Guest, code: u8) {
    set(guest, code);
}

/// Whether `serial` is the pointer's latest enter, and it is still on.
fn current(guest: &Guest, serial: u32) -> bool {
    guest.scene.pointer_on.is_some() && guest.scene.pointer_entered == serial
}

fn set(guest: &mut Guest, code: u8) {
    let scene = &mut guest.scene;
    let (Some(port), Some(handle)) = (lookup_port(b"compositor"), scene.win.out) else { return };
    if scene.cursor == code {
        return;
    }
    let rid = scene.next_serial();
    if cursor_shape(port, rid, handle, code).is_ok() {
        scene.cursor = code;
    }
}
