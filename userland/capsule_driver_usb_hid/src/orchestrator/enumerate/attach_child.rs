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

//! A device on a hub port whose reset is over, addressed through its route
//! and classified like one on a root port (Linux `hub_port_init`).

use crate::hub::{child_route, child_tt, Hub, HubError, Place};
use crate::xhci::{address_routed, disable_slot, enable_slot};

use super::classify::classify;
use super::devices::Devices;

/// The slot kept for the device, or None when nothing on it bound and the
/// slot was given back.
pub(super) fn attach_child(
    xhci_port: u32,
    hub: &Hub,
    port: u8,
    speed: u8,
    devs: &mut Devices,
) -> Result<Option<u8>, HubError> {
    let route = child_route(hub.place.route, hub.place.depth, port).ok_or(HubError::TooDeep)?;
    let place = Place {
        root_port: hub.place.root_port,
        route,
        depth: hub.place.depth + 1,
        speed,
        tt: child_tt(hub.place.speed, hub.slot, hub.place.tt, port, speed),
    };
    let slot = enable_slot(xhci_port).map_err(|_| HubError::NoSlot)?;
    if let Err(e) = address_routed(xhci_port, slot, &place) {
        disable_slot(xhci_port, slot);
        return Err(HubError::Address(e));
    }
    let _ = classify(xhci_port, slot, place, devs);
    if devs.holds(slot) {
        return Ok(Some(slot));
    }
    disable_slot(xhci_port, slot);
    Ok(None)
}
