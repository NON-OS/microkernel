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

//! A destroy request: the object leaves the table, what the scene kept for
//! it goes with it, and the client is told the id is free again. A pool
//! stays while a buffer made from it does, as Wayland keeps its memory.

use crate::linux::guest::Guest;

use super::object::Object;
use super::window_life::End;

pub fn forget(guest: &mut Guest, what: Option<Object>, id: u32) {
    guest.objects.drop_id(id);
    if what == Some(Object::Surface) {
        // The window its pixels made goes with it.
        super::close_window::close_for(&mut guest.scene, End::Surface(id));
    }
    let scene = &mut guest.scene;
    match what {
        Some(Object::Buffer) => scene.buffers.retain(|b| b.id != id),
        Some(Object::Surface) => scene.surfaces.retain(|s| s.id != id),
        _ => {}
    }
    /* A pool goes once its object is gone and no buffer is made from it. */
    let objects = &guest.objects;
    let buffers = &scene.buffers;
    scene.pools.retain(|p| {
        objects.get(p.id) == Some(Object::ShmPool) || buffers.iter().any(|b| b.pool == p.id)
    });
    super::handlers::delete_id(guest, id);
}
