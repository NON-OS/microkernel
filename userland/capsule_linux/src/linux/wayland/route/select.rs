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

//! The primary selection and cursor shapes. A primary selection a client
//! sets is kept as its guest's own and offered to every primary selection
//! device the guest holds, as the data device's selection is
//! (data_device.rs); it never leaves the guest, not even for the desktop
//! clipboard, and no guest is offered another's. The cursor is the
//! compositor's own, so a shape a guest asks for changes nothing. The
//! offers of both selections are served last (clip/ask.rs). False means
//! nothing served it.

use crate::linux::guest::Guest;

use super::super::clip::{ask, which::Which};
use super::super::ops::select::*;
use super::super::{args::Args, forget::forget, object::Object};

pub fn route(
    guest: &mut Guest,
    object: Option<Object>,
    id: u32,
    op: u16,
    args: &mut Args<'_>,
) -> bool {
    use Object::*;
    match (object, op) {
        (Some(PrimaryManager), PRIMARY_MANAGER_CREATE_SOURCE) => {
            super::rest::make(guest, args, PrimarySource)
        }
        (Some(PrimaryManager), PRIMARY_MANAGER_GET_DEVICE) => {
            made_for(guest, args, PrimaryDevice, Seat)
        }
        (Some(PrimaryDevice), PRIMARY_DEVICE_SET_SELECTION) => {
            ask::set(guest, Which::Primary, args)
        }
        (Some(PrimarySource), PRIMARY_SOURCE_OFFER) => ask::mime(guest, id, args),
        (Some(PrimarySource), PRIMARY_SOURCE_DESTROY) => {
            ask::source_gone(guest, Which::Primary, id)
        }
        (Some(CursorManager), CURSOR_MANAGER_GET_POINTER) => {
            made_for(guest, args, CursorDevice, Pointer)
        }
        (Some(CursorDevice), CURSOR_DEVICE_SET_SHAPE) => {}
        (Some(PrimaryManager), PRIMARY_MANAGER_DESTROY)
        | (Some(PrimaryDevice), PRIMARY_DEVICE_DESTROY)
        | (Some(CursorManager), CURSOR_MANAGER_DESTROY)
        | (Some(CursorDevice), CURSOR_DEVICE_DESTROY) => forget(guest, object, id),
        _ => return ask::offer(guest, object, id, op, args),
    }
    true
}

/// `what`, made only when the object it names is a `of`.
fn made_for(guest: &mut Guest, args: &mut Args<'_>, what: Object, of: Object) {
    let (Some(id), Some(named)) = (args.u32(), args.u32()) else { return };
    if guest.objects.get(named) == Some(of) {
        let _ = guest.objects.put(id, what);
    }
}
