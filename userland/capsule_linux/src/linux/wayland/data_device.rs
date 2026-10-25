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

//! `wl_data_device_manager`, minimal and complete. NONOS carries no
//! clipboard or drag between guests, or to the desktop, by design: a guest
//! cannot read what another program copied. A client is given its sources,
//! and a device per seat, and a selection or drag it offers is cancelled at
//! once, so it frees the source and carries nothing.

use crate::linux::guest::Guest;

use super::args::Args;
use super::object::Object;
use super::ops::data::*;
use super::out::Event;

pub fn route(
    guest: &mut Guest,
    object: Option<Object>,
    id: u32,
    op: u16,
    args: &mut Args<'_>,
) -> bool {
    match (object, op) {
        (Some(Object::DataDeviceManager), MANAGER_CREATE_DATA_SOURCE) => {
            make(guest, args, Object::DataSource)
        }
        (Some(Object::DataDeviceManager), MANAGER_GET_DATA_DEVICE) => {
            make(guest, args, Object::DataDevice)
        }
        (Some(Object::DataSource), SOURCE_OFFER | SOURCE_SET_ACTIONS) => {}
        (Some(Object::DataDevice), DEVICE_START_DRAG | DEVICE_SET_SELECTION) => cancel(guest, args),
        (Some(Object::DataSource), SOURCE_DESTROY) | (Some(Object::DataDevice), DEVICE_RELEASE) => {
            super::forget::forget(guest, object, id)
        }
        _ => return super::decoration::route(guest, object, id, op, args),
    }
    true
}

/// A source; or a device, which names the seat it belongs to.
fn make(guest: &mut Guest, args: &mut Args<'_>, what: Object) {
    let Some(id) = args.u32() else { return };
    if what == Object::DataDevice
        && args.u32().and_then(|s| guest.objects.get(s)) != Some(Object::Seat)
    {
        return;
    }
    let _ = guest.objects.put(id, what);
}

/// Both name the source first; null, 0, offers nothing.
fn cancel(guest: &mut Guest, args: &mut Args<'_>) {
    let source = args.u32().unwrap_or(0);
    if guest.objects.get(source) == Some(Object::DataSource) {
        Event::new(source, SOURCE_CANCELLED).send(&mut guest.display.to_client);
    }
}
