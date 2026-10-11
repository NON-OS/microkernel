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
//! as focus_step.rs decides them: onto the shown toplevel, or a grabbing
//! popup, the keyboard's topmost and the pointer's the one under it.

use crate::linux::guest::Guest;

use super::super::out::Event;
use super::leave::{move_from, KEYBOARD_LEAVE, POINTER_LEAVE};

const POINTER_ENTER: u16 = 0;
const KEYBOARD_ENTER: u16 = 1;

fn fixed(value: i32) -> u32 {
    (value << 8) as u32
}

/// The pointer at (`x`, `y`) on the window: the point on the surface it is
/// on, or None when the event is dropped.
pub fn pointer_on(guest: &mut Guest, pointer: u32, x: i32, y: i32) -> Option<(i32, i32)> {
    let shown = guest.scene.shown.map(|w| w.surface);
    let under = guest.scene.popups.under(x, y);
    let (target, (x, y)) = under.map_or((shown, (x, y)), |(popup, at)| (Some(popup), at));
    let on = match move_from(guest, pointer, POINTER_LEAVE, (guest.scene.pointer_on, target)) {
        Ok(on) => on,
        Err(goes_on) => return goes_on.then_some((x, y)),
    };
    let serial = guest.scene.next_serial();
    Event::new(pointer, POINTER_ENTER)
        .u32(serial)
        .u32(on)
        .u32(fixed(x))
        .u32(fixed(y))
        .send(&mut guest.display.to_client);
    guest.scene.pointer_on = Some(on);
    Some((x, y))
}

/// The keys array is empty: nothing is held down at the moment a client is
/// told it has focus, and claiming otherwise would leave it with a key stuck
/// until the matching release it never saw. A leave lets go of every key,
/// so none is held after it.
pub fn keyboard_on(guest: &mut Guest, keyboard: u32) -> bool {
    let shown = guest.scene.shown.map(|w| w.surface);
    let target = guest.scene.popups.top().map(|p| p.surface).or(shown);
    let on = match move_from(guest, keyboard, KEYBOARD_LEAVE, (guest.scene.keyboard_on, target)) {
        Ok(on) => on,
        Err(goes_on) => return goes_on,
    };
    guest.scene.held = super::super::held_keys::HeldKeys::new();
    let serial = guest.scene.next_serial();
    Event::new(keyboard, KEYBOARD_ENTER)
        .u32(serial)
        .u32(on)
        .u32(0)
        .send(&mut guest.display.to_client);
    guest.scene.keyboard_on = Some(on);
    true
}
