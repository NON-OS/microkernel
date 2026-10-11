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

//! Which handler a request belongs to. False means nothing served it.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::destructor::is_destructor;
use super::super::object::Object;
use super::super::ops::req;
use super::super::{buffer, commit, handlers, pool_resize, region, seat, shm, subsurface, surface};

pub fn route(
    guest: &mut Guest,
    object: Option<Object>,
    id: u32,
    opcode: u16,
    args: &mut Args<'_>,
) -> bool {
    match (object, opcode) {
        (Some(Object::Display), req::DISPLAY_SYNC) => handlers::sync(guest, args),
        (Some(Object::Display), req::DISPLAY_GET_REGISTRY) => handlers::registry(guest, args),
        (Some(Object::Registry), req::REGISTRY_BIND) => super::super::bind::bind(guest, id, args),
        (Some(Object::Compositor), req::COMPOSITOR_CREATE_SURFACE) => surface::create(guest, args),
        (Some(Object::Shm), req::SHM_CREATE_POOL) => shm::create_pool(guest, args),
        (Some(Object::ShmPool), req::POOL_CREATE_BUFFER) => buffer::create_buffer(guest, id, args),
        (Some(Object::ShmPool), req::POOL_RESIZE) => pool_resize::resize(guest, id, args),
        (Some(Object::Surface), req::SURFACE_ATTACH) => surface::attach(guest, id, args),
        (Some(Object::Surface), req::SURFACE_FRAME) => surface::frame(guest, id, args),
        (Some(Object::Surface), req::SURFACE_COMMIT) => commit::commit(guest, id),
        (Some(Object::Surface), req::SURFACE_DAMAGE | req::SURFACE_DAMAGE_BUFFER) => {}
        /* Nothing a surface says about its own pixels changes how they are shown (region.rs). */
        (
            Some(Object::Surface),
            req::SURFACE_SET_OPAQUE_REGION
            | req::SURFACE_SET_INPUT_REGION
            | req::SURFACE_SET_BUFFER_TRANSFORM
            | req::SURFACE_SET_BUFFER_SCALE,
        ) => {}
        (Some(Object::Compositor), req::COMPOSITOR_CREATE_REGION) => region::create(guest, args),
        (Some(Object::Region), req::REGION_ADD | req::REGION_SUBTRACT) => {}
        (Some(Object::Subcompositor), req::SUBCOMPOSITOR_GET_SUBSURFACE) => {
            subsurface::get::get(guest, args)
        }
        (Some(Object::Subsurface), req::SUBSURFACE_SET_POSITION..=req::SUBSURFACE_SET_DESYNC) => {
            return subsurface::request::route(guest, id, opcode, args)
        }
        (Some(Object::Seat), req::SEAT_GET_POINTER) => seat::get_pointer(guest, args),
        (Some(Object::Seat), req::SEAT_GET_KEYBOARD) => seat::get_keyboard(guest, args),
        (o, c) if is_destructor(o, c) => super::super::forget::forget(guest, o, id),
        _ => return super::xdg::route(guest, object, id, opcode, args),
    }
    true
}
