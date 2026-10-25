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

//! A popup's grab ended. Every popup from it up is told popup_done,
//! topmost first, and taken off the window, and the keyboard goes at once
//! to what is then on top: the next popup, or the toplevel.
//!
//! end_all is the one a focus-out from the input router will call (I-C8).
//! Today the toplevel's minimize, its full-screen and maximize changes, a
//! buffer of a new shape, and its destruction call it.

use crate::linux::guest::Guest;

use super::super::input::enter::keyboard_on;
use super::super::ops::shell::POPUP_DONE;
use super::super::out::Event;
use super::super::subsurface::compose::repaint;

/// Popup `id`'s grab ends, and every grab above it.
pub fn end(guest: &mut Guest, id: u32) {
    let gone = guest.scene.popups.ending(id);
    finish(guest, &gone, None);
}

/// Every popup ends: the window's grabs are over.
pub fn end_all(guest: &mut Guest) {
    // A grab after this needs a press made after it.
    guest.scene.popups.last_press = None;
    let gone = guest.scene.popups.all();
    finish(guest, &gone, None);
}

pub fn finish(guest: &mut Guest, gone: &[u32], quiet: Option<u32>) {
    if gone.is_empty() {
        return;
    }
    for &id in gone {
        if Some(id) != quiet {
            Event::new(id, POPUP_DONE).send(&mut guest.display.to_client);
        }
        let list = &mut guest.scene.popups.list;
        if let Some(at) = list.iter().position(|p| p.id == id) {
            let p = list.remove(at);
            super::layer::take(&mut guest.scene, p.surface);
        }
    }
    if let Some(keyboard) = guest.scene.keyboard {
        let _ = keyboard_on(guest, keyboard);
    }
    repaint(guest);
}
