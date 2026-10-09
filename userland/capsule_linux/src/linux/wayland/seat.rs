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
}

pub fn get_keyboard(guest: &mut Guest, args: &mut Args<'_>) {
    let Some(id) = args.u32() else { return };
    if !guest.objects.put(id, Object::Keyboard) {
        return;
    }
    guest.scene.keyboard = Some(id);
    super::keymap::send(guest, id);
}
