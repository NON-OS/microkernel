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

//! Where a device below a hub is, from the client's route, speed and TT,
//! each held against the hubs this driver set up: the route names a port
//! of a hub Configure Hub set up on that root port, no more than five
//! tiers down; the speed is one that hub's ports carry; and the TT is the
//! one the hub's place implies (xHCI 1.2 section 6.2.2, as recalled,
//! unverified), or none.

use crate::contexts::{
    parent_of, route_depth, Place, Topology, SPEED_FULL, SPEED_HIGH, SPEED_LOW, SPEED_SUPER,
};
use crate::server::context::Context;
use crate::slots::SlotResources;

pub fn place_below_hub(
    ctx: &Context,
    root_port: u8,
    speed: u8,
    route: u32,
    tt: (u8, u8),
) -> Option<Place> {
    let depth = route_depth(route).filter(|&d| d >= 1)?;
    let (hub_route, port) = parent_of(route, depth);
    let hub = ctx.driver.slots.hub_at(root_port, hub_route)?;
    let ports = hub.hub.map_or(0, |h| h.ports);
    if port == 0 || port > ports || !carries(hub.speed, speed) {
        return None;
    }
    let topo = topology(hub, port, speed, route);
    ((topo.tt_slot, topo.tt_port) == tt).then_some(Place { root_port, speed, topo })
}

/// Whether a hub at `hub` speed has ports a device at `speed` can be on:
/// SuperSpeed below SuperSpeed, up to high speed below high speed, low and
/// full speed below full speed. Speed ids are xHCI's defaults, 4 bits.
fn carries(hub: u8, speed: u8) -> bool {
    match hub {
        SPEED_FULL => matches!(speed, SPEED_FULL | SPEED_LOW),
        SPEED_HIGH => matches!(speed, SPEED_FULL | SPEED_LOW | SPEED_HIGH),
        SPEED_SUPER..=15 => (SPEED_SUPER..=15).contains(&speed),
        _ => false,
    }
}

/// A low or full speed device's TT: the high-speed hub it is on, or the
/// one the full speed hub it is on is reached through; none otherwise.
fn topology(hub: &SlotResources, port: u8, speed: u8, route: u32) -> Topology {
    let none = Topology { route, ..Topology::default() };
    if !matches!(speed, SPEED_FULL | SPEED_LOW) {
        return none;
    }
    if hub.speed == SPEED_HIGH {
        let mtt = hub.hub.is_some_and(|h| h.mtt);
        return Topology { route, tt_slot: hub.slot_id, tt_port: port, mtt };
    }
    Topology { route, ..hub.topo }
}
