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
use super::super::popup::corner::on_surface;
use super::devices::{keyboards, pointers};
use super::leave::{move_from, KEYBOARD_LEAVE, POINTER_LEAVE};

const POINTER_ENTER: u16 = 0;
const KEYBOARD_ENTER: u16 = 1;
const KEYBOARD_MODIFIERS: u16 = 4;

fn fixed(value: i32) -> u32 {
    (value << 8) as u32
}

/// The pointer at (`x`, `y`) on the window: the point on the surface it is
/// on, or None when the event is dropped. Every pointer the client holds is
/// told the same (devices.rs).
pub fn pointer_on(guest: &mut Guest, x: i32, y: i32) -> Option<(i32, i32)> {
    let pointers = pointers(guest);
    let shown = guest.scene.win.shown.map(|w| w.surface);
    let under = guest.scene.popups.under(x, y);
    let on_popup = |(popup, at)| (Some(popup), on_surface(&guest.scene, popup, at));
    let (target, (x, y)) = under.map_or((shown, (x, y)), on_popup);
    let was = guest.scene.pointer_on;
    let on = match move_from(guest, &pointers, POINTER_LEAVE, (was, target)) {
        Ok(on) => on,
        Err(goes_on) => return goes_on.then_some((x, y)),
    };
    let serial = guest.scene.next_serial();
    (guest.scene.pointer_on, guest.scene.pointer_entered) = (Some(on), serial);
    pointers.into_iter().for_each(|p| pointer_enter(guest, p, (on, serial), (x, y)));
    Some((x, y))
}

/// wl_pointer.enter on `pointer`, onto `on` at (`x`, `y`) under `serial`.
pub fn pointer_enter(guest: &mut Guest, pointer: u32, (on, serial): (u32, u32), (x, y): (i32, i32)) {
    guest.scene.pointer_at = (x, y);
    let ev = Event::new(pointer, POINTER_ENTER).u32(serial).u32(on).u32(fixed(x)).u32(fixed(y));
    ev.send(&mut guest.display.to_client);
}

/// The keys array is empty: nothing is held down at the moment a client is
/// told it has focus, and claiming otherwise would leave it with a key stuck
/// until the matching release it never saw. A leave lets go of every key,
/// so none is held after it. The modifiers follow at once, as wl_keyboard
/// requires: none held, so a client that kept Ctrl from before its leave
/// (Qt does) does not take the next key as Ctrl and that key.
pub fn keyboard_on(guest: &mut Guest) -> bool {
    let keyboards = keyboards(guest);
    let shown = guest.scene.win.shown.map(|w| w.surface);
    let target = guest.scene.popups.top().map(|p| p.surface).or(shown);
    let was = guest.scene.keyboard_on;
    let on = match move_from(guest, &keyboards, KEYBOARD_LEAVE, (was, target)) {
        Ok(on) => on,
        Err(goes_on) => return goes_on,
    };
    guest.scene.held = super::super::held_keys::HeldKeys::new();
    super::super::clip::focus::entered(guest);
    guest.scene.keyboard_on = Some(on);
    keyboards.into_iter().for_each(|k| keyboard_enter(guest, k, on));
    true
}

/// wl_keyboard.enter on `keyboard`, onto `on`, nothing held, then the
/// modifiers: none.
pub fn keyboard_enter(guest: &mut Guest, keyboard: u32, on: u32) {
    let serial = guest.scene.next_serial();
    let to = &mut guest.display.to_client;
    Event::new(keyboard, KEYBOARD_ENTER).u32(serial).u32(on).u32(0).send(to);
    let serial = guest.scene.next_serial();
    let to = &mut guest.display.to_client;
    Event::new(keyboard, KEYBOARD_MODIFIERS).u32(serial).u32(0).u32(0).u32(0).u32(0).send(to);
}
