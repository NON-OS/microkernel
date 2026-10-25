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

//! `zxdg_decoration_manager_v1`: who draws a window's frame. Always the
//! server: a guest's window is framed by the NONOS window manager like any
//! other, so a client is told server-side and draws none of its own. The
//! first answer is a configure sequence, ended by its xdg_surface's
//! configure, whose serial the client acks.

use crate::linux::guest::Guest;

use super::args::Args;
use super::object::Object;
use super::ops::decoration::*;
use super::ops::ev;
use super::out::Event;

pub fn route(
    guest: &mut Guest,
    object: Option<Object>,
    id: u32,
    op: u16,
    args: &mut Args<'_>,
) -> bool {
    match (object, op) {
        (Some(Object::DecorationManager), MANAGER_GET_TOPLEVEL_DECORATION) => get(guest, args),
        /* The mode never changes, so it is said again with no new serial;
         * the next xdg_surface configure ends the sequence it joins. */
        (Some(Object::Decoration), DECORATION_SET_MODE | DECORATION_UNSET_MODE) => {
            Event::new(id, DECORATION_CONFIGURE).u32(SERVER_SIDE).send(&mut guest.display.to_client)
        }
        (Some(Object::DecorationManager), MANAGER_DESTROY)
        | (Some(Object::Decoration), DECORATION_DESTROY) => {
            super::forget::forget(guest, object, id)
        }
        _ => return super::route::rest::route(guest, object, id, op, args),
    }
    true
}

/// Made only for a toplevel whose xdg_surface ends the sequence.
fn get(guest: &mut Guest, args: &mut Args<'_>) {
    let (Some(id), Some(toplevel)) = (args.u32(), args.u32()) else { return };
    let owner = guest.scene.surfaces.iter().find(|s| s.toplevel == Some(toplevel));
    let Some(xdg) = owner.and_then(|s| s.xdg) else { return };
    if !guest.objects.put(id, Object::Decoration) {
        return;
    }
    let serial = guest.scene.next_serial();
    let to_client = &mut guest.display.to_client;
    Event::new(id, DECORATION_CONFIGURE).u32(SERVER_SIDE).send(to_client);
    Event::new(xdg, ev::XDG_SURFACE_CONFIGURE).u32(serial).send(to_client);
}
