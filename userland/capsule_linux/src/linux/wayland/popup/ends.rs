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
//! to what is then on top, the next popup or the toplevel, while the window
//! has focus.
//!
//! end_all is the one a focus-out from the input router will call (I-C8).
//! Today the toplevel's minimize, its full-screen and maximize changes, a
//! move or a resize from its frame or asked by its client, a buffer of a new
//! shape, and its destruction call it.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::super::input::enter::keyboard_on;
use super::super::input::focus::popups_gone;
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
    guest.scene.popups.forget_input();
    let gone = guest.scene.popups.all();
    finish(guest, &gone, None);
}

pub fn finish(guest: &mut Guest, gone: &[u32], quiet: Option<u32>) {
    if gone.is_empty() {
        return;
    }
    let mut surfaces = Vec::with_capacity(gone.len());
    for &id in gone {
        if Some(id) != quiet {
            Event::new(id, POPUP_DONE).send(&mut guest.display.to_client);
        }
        let list = &mut guest.scene.popups.list;
        if let Some(at) = list.iter().position(|p| p.id == id) {
            let p = list.remove(at);
            surfaces.push(p.surface);
            super::layer::take(&mut guest.scene, p.surface);
            super::window::close(&mut guest.scene, p.id);
        }
    }
    // The keyboard goes to what is on top only while the window has focus;
    // without it, it only leaves a popup that ended under it.
    match guest.scene.win.focused {
        true => {
            let _ = keyboard_on(guest);
        }
        false => popups_gone(guest, &surfaces),
    }
    repaint(guest);
}
