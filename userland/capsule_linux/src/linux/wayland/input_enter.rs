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

//! The enter and leave events a client needs before it will act on input,
//! as focus_step.rs decides them. Each says false when the event is to be
//! dropped, no window being shown.

use crate::linux::guest::Guest;

use super::input_leave::{move_from, KEYBOARD_LEAVE, POINTER_LEAVE};
use super::out::Event;

const POINTER_ENTER: u16 = 0;
const KEYBOARD_ENTER: u16 = 1;

fn fixed(value: i32) -> u32 {
    (value << 8) as u32
}

pub fn pointer_on(guest: &mut Guest, pointer: u32, x: i32, y: i32) -> bool {
    let on = match move_from(guest, pointer, POINTER_LEAVE, guest.scene.pointer_on) {
        Ok(on) => on,
        Err(goes_on) => return goes_on,
    };
    let serial = guest.scene.next_serial();
    Event::new(pointer, POINTER_ENTER)
        .u32(serial)
        .u32(on)
        .u32(fixed(x))
        .u32(fixed(y))
        .send(&mut guest.display.to_client);
    guest.scene.pointer_on = Some(on);
    true
}

/// The keys array is empty: nothing is held down at the moment a client is
/// told it has focus, and claiming otherwise would leave it with a key stuck
/// until the matching release it never saw. A leave lets go of every key,
/// so none is held after it.
pub fn keyboard_on(guest: &mut Guest, keyboard: u32) -> bool {
    let on = match move_from(guest, keyboard, KEYBOARD_LEAVE, guest.scene.keyboard_on) {
        Ok(on) => on,
        Err(goes_on) => return goes_on,
    };
    guest.scene.held = super::held_keys::HeldKeys::new();
    let serial = guest.scene.next_serial();
    Event::new(keyboard, KEYBOARD_ENTER)
        .u32(serial)
        .u32(on)
        .u32(0)
        .send(&mut guest.display.to_client);
    guest.scene.keyboard_on = Some(on);
    true
}
