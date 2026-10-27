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

//! `wl_data_device_manager`. A selection a client sets is kept as its
//! guest's own and offered to every data device the guest holds (clip/);
//! no guest is ever offered another guest's. Text is carried to and from
//! the user's desktop clipboard, only for the window with focus
//! (clip/desk.rs). A drag is cancelled at once, so its source is freed and
//! carries nothing.

use crate::linux::guest::Guest;

use super::args::Args;
use super::clip::{ask, which::Which};
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
        (Some(Object::DataSource), SOURCE_OFFER) => ask::mime(guest, id, args),
        (Some(Object::DataSource), SOURCE_SET_ACTIONS) => {}
        (Some(Object::DataDevice), DEVICE_SET_SELECTION) => ask::set(guest, Which::Data, args),
        (Some(Object::DataDevice), DEVICE_START_DRAG) => cancel(guest, args),
        (Some(Object::DataSource), SOURCE_DESTROY) => ask::source_gone(guest, Which::Data, id),
        (Some(Object::DataDevice), DEVICE_RELEASE) => super::forget::forget(guest, object, id),
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

/// A drag names its source first; null, 0, offers nothing.
fn cancel(guest: &mut Guest, args: &mut Args<'_>) {
    let source = args.u32().unwrap_or(0);
    if guest.objects.get(source) == Some(Object::DataSource) {
        Event::new(source, SOURCE_CANCELLED).send(&mut guest.display.to_client);
    }
}
