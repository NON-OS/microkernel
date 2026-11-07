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

//! A pointer button, written to the client.

use crate::linux::guest::Guest;

use super::super::out::Event;
use super::enter::pointer_on;
use super::send::{frame, time_ms};

/// wl_pointer.button.
const POINTER_BUTTON: u16 = 3;

/// A press at (`x`, `y`) on the toplevel's surface. The client takes a
/// button as on the surface it last entered, so the pointer is first put on
/// what is under the press: a popup mapped under a pointer that has not moved
/// since would otherwise read the press as one outside it and close. The
/// enter, when there is one, is its own frame; nothing more is sent when the
/// press is on the surface entered.
pub fn press_at(guest: &mut Guest, button: u32, (x, y): (i32, i32)) {
    let entered = guest.scene.pointer_entered;
    let _ = pointer_on(guest, x, y);
    if guest.scene.pointer_entered != entered {
        for id in super::devices::pointers(guest) {
            frame(guest, id);
        }
    }
    self::button(guest, button, 1);
}

/// `button` is the driver's, from 1; one with no Linux code sends nothing,
/// and nor does one before a motion has put the pointer on the shown
/// surface, the button carrying no place to enter it at.
pub fn button(guest: &mut Guest, button: u32, state: u32) {
    let shown = guest.scene.win.shown.map(|w| w.surface);
    let popup = |on: u32| guest.scene.popups.list.iter().any(|p| p.shown && p.surface == on);
    if !guest.scene.pointer_on.is_some_and(|on| Some(on) == shown || popup(on)) {
        return;
    }
    let Some(code) = super::super::pointer_codes::button_code(button) else { return };
    let serial = guest.scene.next_serial();
    if state == 1 {
        guest.scene.popups.pressed(serial);
    }
    let time = time_ms();
    for id in super::devices::pointers(guest) {
        let ev = Event::new(id, POINTER_BUTTON).u32(serial).u32(time).u32(code).u32(state);
        ev.send(&mut guest.display.to_client);
        frame(guest, id);
    }
}
