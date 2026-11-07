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

//! The drag over the window: each device entered on the surface under it
//! with a new offer of the source's types and actions, told the motion while
//! it stays, and left as it goes. Over the frame, or off the window, it is
//! over nothing.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::super::clip::offer::server_id;
use super::super::object::Object;
use super::super::ops::data::*;
use super::super::out::Event;

/// The drag is over `on`: a surface and the point on it, or nothing.
pub fn motion(guest: &mut Guest, on: Option<(u32, i32, i32)>) {
    let Some(drag) = guest.scene.dnd.drag.as_ref().filter(|d| !d.dropped) else { return };
    let was = drag.over;
    match on {
        Some((surface, x, y)) if was == Some(surface) => moved(guest, x, y),
        Some((surface, x, y)) => {
            leave(guest);
            enter(guest, surface, x, y);
        }
        None => leave(guest),
    }
}

fn fixed(v: i32) -> u32 {
    (v << 8) as u32
}

fn moved(guest: &mut Guest, x: i32, y: i32) {
    let time = super::super::input::send::time_ms();
    let devices: Vec<u32> = guest.objects.all(Object::DataDevice).collect();
    let to = &mut guest.display.to_client;
    for device in devices {
        Event::new(device, DEVICE_MOTION).u32(time).u32(fixed(x)).u32(fixed(y)).send(to);
    }
}

/// Each device entered on `surface`, given an offer when there is a source.
fn enter(guest: &mut Guest, surface: u32, x: i32, y: i32) {
    let devices: Vec<u32> = guest.objects.all(Object::DataDevice).collect();
    let Some(source) = guest.scene.dnd.drag.as_ref().map(|d| d.source) else { return };
    let actions = guest.scene.dnd.offered(source);
    let mut offers = Vec::new();
    for device in devices {
        let offer = source.and_then(|s| offer(guest, device, s, actions));
        let serial = guest.scene.next_serial();
        Event::new(device, DEVICE_ENTER)
            .u32(serial)
            .u32(surface)
            .u32(fixed(x))
            .u32(fixed(y))
            .u32(offer.unwrap_or(0))
            .send(&mut guest.display.to_client);
        offers.extend(offer.map(|o| (device, o)));
    }
    if let Some(d) = guest.scene.dnd.drag.as_mut() {
        (d.over, d.offers, d.accepted, d.dest, d.action) =
            (Some(surface), offers, false, (0, 0), 0);
    }
}

/// One offer of `source` to `device`, at the device's version: its types,
/// then (version 3) the source's actions. Kept with the guest's offers, so
/// a receive on it reaches the source as a selection's does.
fn offer(guest: &mut Guest, device: u32, source: u32, actions: u32) -> Option<u32> {
    let id = server_id(&guest.objects);
    let version = guest.objects.version(device);
    if !guest.objects.bound(id, Object::DataOffer, version) {
        return None;
    }
    guest.scene.clip.offers.push((id, source));
    let to = &mut guest.display.to_client;
    Event::new(device, DEVICE_DATA_OFFER).u32(id).send(to);
    for mime in guest.scene.clip.mimes_of(source) {
        Event::new(id, OFFER_OFFER).string(&mime).send(to);
    }
    if version >= 3 {
        Event::new(id, OFFER_SOURCE_ACTIONS).u32(actions).send(to);
    }
    Some(id)
}

/// Each device left, and the source told no one takes it now.
pub fn leave(guest: &mut Guest) {
    let Some(d) = guest.scene.dnd.drag.as_mut() else { return };
    if d.over.take().is_none() {
        return;
    }
    let (source, accepted) = (d.source, core::mem::take(&mut d.accepted));
    d.offers.clear();
    let devices: Vec<u32> = guest.objects.all(Object::DataDevice).collect();
    let to = &mut guest.display.to_client;
    devices.into_iter().for_each(|device| Event::new(device, DEVICE_LEAVE).send(to));
    if let (Some(source), true) = (source, accepted) {
        Event::new(source, SOURCE_TARGET).u32(0).send(to);
    }
}
