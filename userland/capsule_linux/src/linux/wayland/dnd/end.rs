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

//! A drag's end. The button let go over a client that accepted a type and
//! an action is a drop: the devices are told, the source is told the drop
//! was performed, and the destination reads the data and finishes, after
//! which the source is told the drag is done. Let go anywhere else, or its
//! source destroyed, the drag is cancelled. Either way the pointer is free,
//! and its next motion enters the surface under it again.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::super::object::Object;
use super::super::ops::data::*;
use super::super::out::Event;

/// The button let go during a drag.
pub fn release(guest: &mut Guest) {
    let Some(d) = guest.scene.dnd.drag.as_ref().filter(|d| !d.dropped) else { return };
    let source_v3 = d.source.is_none_or(|s| guest.objects.at_least(s, 3));
    let agreed = d.action != super::action::NONE || !source_v3;
    let lands = d.over.is_some() && (d.source.is_none() || (d.accepted && agreed));
    if !lands {
        return cancel(guest);
    }
    let devices: Vec<u32> = guest.objects.all(Object::DataDevice).collect();
    let to = &mut guest.display.to_client;
    devices.into_iter().for_each(|device| Event::new(device, DEVICE_DROP).send(to));
    let source = d.source;
    match source {
        Some(s) if guest.objects.at_least(s, 3) => {
            Event::new(s, SOURCE_DND_DROP_PERFORMED).send(to);
            if let Some(d) = guest.scene.dnd.drag.as_mut() {
                d.dropped = true;
            }
        }
        // Before version 3 there is no finish to wait for.
        _ => guest.scene.dnd.drag = None,
    }
}

/// The drag cancelled: left, and its source told.
pub fn cancel(guest: &mut Guest) {
    super::over::leave(guest);
    let Some(d) = guest.scene.dnd.drag.take() else { return };
    if let Some(source) = d.source.filter(|&s| guest.objects.get(s) == Some(Object::DataSource)) {
        Event::new(source, SOURCE_CANCELLED).send(&mut guest.display.to_client);
    }
}

/// wl_data_offer.finish on the dropped drag's offer: the source is done.
pub fn finish(guest: &mut Guest, offer: u32) {
    let done = guest
        .scene
        .dnd
        .drag
        .as_ref()
        .is_some_and(|d| d.dropped && d.offers.iter().any(|o| o.1 == offer));
    if !done {
        return;
    }
    if let Some(source) = guest.scene.dnd.drag.take().and_then(|d| d.source) {
        Event::new(source, SOURCE_DND_FINISHED).send(&mut guest.display.to_client);
    }
}

/// The source destroyed: a drag of it ends, with nothing more told to it.
pub fn source_gone(guest: &mut Guest, source: u32) {
    guest.scene.dnd.actions.retain(|a| a.0 != source);
    if guest.scene.dnd.drag.as_ref().is_some_and(|d| d.source == Some(source)) {
        super::over::leave(guest);
        guest.scene.dnd.drag = None;
    }
}
