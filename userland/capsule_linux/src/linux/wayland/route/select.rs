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

//! The primary selection and cursor shapes. NONOS carries no selection
//! between guests, or to the desktop, by design, as the data device does
//! not (data_device.rs): a guest is given its sources and a device per
//! seat, and a selection it offers is cancelled at once. The cursor is the
//! compositor's own, so a shape a guest asks for changes nothing. False
//! means nothing served it.

use crate::linux::guest::Guest;

use super::super::ops::select::*;
use super::super::out::Event;
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
        (Some(PrimaryDevice), PRIMARY_DEVICE_SET_SELECTION) => cancel(guest, args),
        (Some(PrimarySource), PRIMARY_SOURCE_OFFER) => {}
        (Some(CursorManager), CURSOR_MANAGER_GET_POINTER) => {
            made_for(guest, args, CursorDevice, Pointer)
        }
        (Some(CursorDevice), CURSOR_DEVICE_SET_SHAPE) => {}
        (Some(PrimaryManager), PRIMARY_MANAGER_DESTROY)
        | (Some(PrimaryDevice), PRIMARY_DEVICE_DESTROY)
        | (Some(PrimarySource), PRIMARY_SOURCE_DESTROY)
        | (Some(CursorManager), CURSOR_MANAGER_DESTROY)
        | (Some(CursorDevice), CURSOR_DEVICE_DESTROY) => forget(guest, object, id),
        _ => return false,
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

fn cancel(guest: &mut Guest, args: &mut Args<'_>) {
    let source = args.u32().unwrap_or(0);
    if guest.objects.get(source) == Some(Object::PrimarySource) {
        Event::new(source, PRIMARY_SOURCE_CANCELLED).send(&mut guest.display.to_client);
    }
}
