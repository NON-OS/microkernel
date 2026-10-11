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

//! `wl_surface.commit`: the only request that reaches the screen.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::ops::ev;
use super::out::Event;
use super::present::present;

/// A commit with a buffer is the only thing that reaches the screen, and
/// only a window's: the surface of a toplevel. Any other surface, a
/// subsurface or a cursor, has its buffer released and its frames answered,
/// so its client draws on, and never takes the window's place.
pub fn commit(guest: &mut Guest, id: u32) {
    let Some((buffer, window)) = surface_buffer(guest, id) else {
        return;
    };
    if window {
        present(guest, id, buffer);
        super::output_enter::shown(guest);
    }
    Event::new(buffer, ev::BUFFER_RELEASE).send(&mut guest.display.to_client);
    let owed = take_frames(guest, id);
    for callback in owed {
        Event::new(callback, ev::CALLBACK_DONE).u32(0).send(&mut guest.display.to_client);
        /* A callback ends with its done, and its id goes back to the client. */
        guest.objects.drop_id(callback);
        super::handlers::delete_id(guest, callback);
    }
}

fn surface_buffer(guest: &Guest, id: u32) -> Option<(u32, bool)> {
    let s = guest.scene.surfaces.iter().find(|s| s.id == id)?;
    Some((s.pending?, s.toplevel.is_some()))
}

fn take_frames(guest: &mut Guest, id: u32) -> Vec<u32> {
    match guest.scene.surfaces.iter_mut().find(|s| s.id == id) {
        Some(s) => core::mem::take(&mut s.frames),
        None => Vec::new(),
    }
}
