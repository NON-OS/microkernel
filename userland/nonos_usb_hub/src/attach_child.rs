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
//! A device newly connected to a hub port: debounced, reset by the hub,
//! addressed through its route, and handed to the caller's classes.

use alloc::vec::Vec;

use crate::classes::Classes;
use crate::error::HubError;
use crate::host::Host;
use crate::node::Hub;
use crate::place::Place;
use crate::reset::reset_port;
use crate::route::{child_route, child_tt};

/// TATTDB, how long a connection must hold before the port is reset (USB
/// 2.0 section 7.1.7.3).
const DEBOUNCE_MS: u64 = 100;

/// The slot kept for the device, or None when nothing on it was kept and
/// the slot was given back.
pub(crate) fn attach_child<H: Host>(
    h: &mut H,
    hub: &Hub,
    port: u8,
    hubs: &mut Vec<Hub>,
    classes: &mut impl Classes<H>,
) -> Result<Option<u8>, HubError> {
    h.idle_ms(DEBOUNCE_MS);
    let st = reset_port(h, hub.slot, port, hub.place.superspeed())?;
    let route = child_route(hub.place.route, hub.place.depth, port).ok_or(HubError::TooDeep)?;
    let place = Place {
        root_port: hub.place.root_port,
        route,
        depth: hub.place.depth + 1,
        speed: st.speed,
        tt: child_tt(hub.place.speed, hub.slot, hub.place.tt, port, st.speed),
        parent: (hub.slot, port),
    };
    let slot = h.enable_slot().map_err(|_| HubError::NoSlot)?;
    if let Err(status) = h.address_routed(slot, &place) {
        h.disable_slot(slot);
        return Err(HubError::Address(status));
    }
    let bound = classes.classify(h, slot, place, hubs);
    if bound || hubs.iter().any(|x| x.slot == slot) {
        return Ok(Some(slot));
    }
    h.disable_slot(slot);
    Ok(None)
}
