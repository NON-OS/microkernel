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

//! `wl_surface`: what the client is drawing, and when it says so.

use crate::linux::guest::Guest;

use super::args::Args;
use super::object::Object;
use super::state::Surface;

pub fn create(guest: &mut Guest, args: &mut Args<'_>) {
    let Some(id) = args.u32() else { return };
    if !guest.objects.put(id, Object::Surface) {
        return;
    }
    guest.scene.surfaces.retain(|s| s.id != id);
    guest.scene.surfaces.push(Surface {
        id,
        pending: None,
        xdg: None,
        toplevel: None,
        frames: alloc::vec::Vec::new(),
        held: alloc::vec::Vec::new(),
        last_done: None,
        configured: false,
        initial: false,
        wants: None,
        parent: false,
        limits: ((0, 0), (0, 0)),
        geometry: None,
        title: alloc::vec::Vec::new(),
        own_frame: false,
        feedback: alloc::vec::Vec::new(),
        scale: Default::default(),
        damage: Default::default(),
        boxes: Default::default(),
        attached: false,
        opaque_said: None,
        opaque: Default::default(),
    });
    super::route::scale_surface::told(guest, id);
}

pub fn attach(guest: &mut Guest, id: u32, args: &mut Args<'_>) {
    let Some(buffer) = args.u32() else { return };
    // An object of another kind is the client's error, as libwayland's
    // typed arguments make it: kept, its release went to that object.
    if buffer != 0 && guest.objects.get(buffer) != Some(Object::Buffer) {
        let message = alloc::format!("object {buffer} is not a wl_buffer");
        return super::display_error::fatal(guest, id, &message);
    }
    if let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == id) {
        s.attached = true;
        s.pending = match buffer {
            0 => None,
            b => Some(b),
        };
    }
}

pub fn frame(guest: &mut Guest, id: u32, args: &mut Args<'_>) {
    let Some(callback) = args.u32() else { return };
    if !guest.objects.put(callback, Object::Callback) {
        return;
    }
    if let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == id) {
        s.frames.push(callback);
    }
}
