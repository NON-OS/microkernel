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


//! `wl_seat`: the pointer and keyboard objects a client asks for.

use crate::linux::guest::Guest;

use super::object::Object;
use super::args::Args;

pub fn get_pointer(guest: &mut Guest, args: &mut Args<'_>) {
    let Some(id) = args.u32() else { return };
    if !guest.objects.put(id, Object::Pointer) {
        return;
    }
    guest.scene.pointer = Some(id);
    // Made while the pointer is on a surface: it is told so at once, as the
    // client's other pointers were (input/enter.rs).
    if let Some(on) = guest.scene.pointer_on {
        let (serial, at) = (guest.scene.pointer_entered, guest.scene.pointer_at);
        super::input::enter::pointer_enter(guest, id, (on, serial), at);
        super::input::send::frame(guest, id);
    }
}

pub fn get_keyboard(guest: &mut Guest, args: &mut Args<'_>) {
    let Some(id) = args.u32() else { return };
    if !guest.objects.put(id, Object::Keyboard) {
        return;
    }
    guest.scene.keyboard = Some(id);
    super::keymap::send(guest, id);
    // Made while the window has the keyboard: told so at once, with nothing
    // down, as the client's other keyboards were (input/enter.rs).
    if let Some(on) = guest.scene.keyboard_on {
        super::input::enter::keyboard_enter(guest, id, on);
    }
}
