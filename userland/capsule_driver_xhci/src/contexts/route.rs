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

//! Where a device sits below its root port (xHCI 1.2 sections 4.3.3, 6.2.2
//! and 8.9): its route string, one 4-bit hub port per tier from bits 3:0,
//! and, for a low or full speed device below a high-speed hub, the hub slot
//! and port whose transaction translator its split transactions go through.

/// Five hubs at most between a root port and a device (USB 2.0 section
/// 4.1.1); the route string has a 4-bit field for each.
pub const MAX_TIERS: u8 = 5;
/// The most ports a route string can name on one hub.
pub const MAX_HUB_PORTS: u8 = 15;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Topology {
    /// The route string; 0 on a root port.
    pub route: u32,
    /// The TT hub's slot, 0 for none, and the hub port the device is on.
    pub tt_slot: u8,
    pub tt_port: u8,
    /// The TT hub runs one TT per port (Slot Context MTT).
    pub mtt: bool,
}

/// The hubs between the root port and the device `route` names, or
/// `None` when it names none: bits past the fifth tier, or a tier left
/// empty below one that is not.
pub fn route_depth(route: u32) -> Option<u8> {
    if route >> (4 * MAX_TIERS as u32) != 0 {
        return None;
    }
    let mut depth = 0u8;
    let mut rest = route;
    while rest & 0xF != 0 {
        depth += 1;
        rest >>= 4;
    }
    (rest == 0).then_some(depth)
}

/// The route of the hub a device at `route`, `depth` tiers down, is
/// attached to, and the port it is on. `depth` is at least 1.
pub fn parent_of(route: u32, depth: u8) -> (u32, u8) {
    let shift = 4 * (depth.clamp(1, MAX_TIERS) as u32 - 1);
    (route & ((1 << shift) - 1), ((route >> shift) & 0xF) as u8)
}
