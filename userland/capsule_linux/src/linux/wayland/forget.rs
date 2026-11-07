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
    super::subsurface::life::gone(guest, what, id);
    super::popup::gone::parent_gone(guest, what, id);
    if what == Some(Object::Buffer) {
        super::present::owed::buffer_gone(guest, id);
        super::present::pinned::show::buffer_gone(guest, id);
    }
    if what == Some(Object::Surface) {
        super::presentation::gone(guest, id);
        // Off every output, told nothing: the id may name a new surface.
        super::output_enter::left(guest, id);
        // The window its pixels made goes with it.
        super::close::close_window::close_for(guest, End::Surface(id));
    }
    let scene = &mut guest.scene;
    match what {
        Some(Object::Buffer) => {
            scene.buffers.retain(|b| b.id != id)
        }
        Some(Object::Decoration | Object::KdeDecoration) => {
            scene.decorations.retain(|d| d.0 != id)
        }
        Some(Object::Surface) => scene.surfaces.retain(|s| s.id != id),
        Some(Object::Region) => scene.regions.retain(|r| r.0 != id),
        // A released pointer or keyboard is sent nothing more.
        // Another the client still holds stands for it.
        Some(Object::Pointer) if scene.pointer == Some(id) => {
            scene.pointer = guest.objects.all(Object::Pointer).last()
        }
        Some(Object::Keyboard) if scene.keyboard == Some(id) => {
            scene.keyboard = guest.objects.all(Object::Keyboard).last()
        }
        _ => {}
    }
    /* A pool goes once its object is gone and no buffer is made from it. */
    if what == Some(Object::ShmPool) {
        super::shm::kept(scene, id);
    }
    super::shm::drop_pools(guest, |g, p| {
        g.objects.get(p.id) == Some(Object::ShmPool) || g.scene.buffers.iter().any(|b| b.pool == p.id)
    });
    super::handlers::delete_id(guest, id);
}
