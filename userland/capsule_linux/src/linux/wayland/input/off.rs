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

//! The pointer off the client's surfaces, onto the window's frame: told it
//! left, then a frame, and the cursor the arrow again. Its next motion on the
//! content enters it anew (enter.rs), so a hover the client drew ends.

use crate::linux::guest::Guest;

use super::super::object::Object;
use super::super::out::Event;
use super::leave::POINTER_LEAVE;

pub fn pointer_off(guest: &mut Guest) {
    let Some(on) = guest.scene.pointer_on.take() else { return };
    if guest.objects.get(on) == Some(Object::Surface) {
        let serial = guest.scene.next_serial();
        for pointer in super::devices::pointers(guest) {
            let ev = Event::new(pointer, POINTER_LEAVE).u32(serial).u32(on);
            ev.send(&mut guest.display.to_client);
            super::send::frame(guest, pointer);
        }
    }
    super::cursor::arrow(guest);
}
