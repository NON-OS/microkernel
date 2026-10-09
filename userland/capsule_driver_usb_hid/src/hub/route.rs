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

//! Where a device sits below the root port: the route string (xHCI 1.2
//! section 8.9, USB 3.2 section 8.9) and the transaction translator a low or
//! full speed device is reached through (xHCI 1.2 section 6.2.2, Linux
//! `xhci_setup_addressable_virt_dev`). Pure, so both are proven on the host.

use super::port_status::{SPEED_FULL, SPEED_HIGH, SPEED_LOW};

/// Five hubs at most between a root port and a device (USB 2.0 section
/// 4.1.1); the route string has a 4-bit field for each.
pub const MAX_HUB_DEPTH: u8 = 5;

/// The high-speed hub port a low or full speed device's split transactions
/// go through.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tt {
    pub hub_slot: u8,
    pub port: u8,
}

/// The route string of the device on `port` of a hub whose own route is
/// `hub_route` and which has `hub_depth` hubs above it (0 on a root port).
/// None past the fifth tier or for port 0, which is the hub itself.
pub fn child_route(hub_route: u32, hub_depth: u8, port: u8) -> Option<u32> {
    if port == 0 || hub_depth >= MAX_HUB_DEPTH {
        return None;
    }
    let field = port.min(15) as u32;
    Some(hub_route | (field << (4 * hub_depth as u32)))
}

/// The TT of a `child_speed` device on `port` of a hub at `hub_speed` in
/// `hub_slot`, whose own TT is `hub_tt`. A high-speed hub translates for a
/// slower device itself; a full speed hub hands down the TT it is reached
/// through; nothing translates for a high or SuperSpeed device.
pub fn child_tt(
    hub_speed: u8,
    hub_slot: u8,
    hub_tt: Option<Tt>,
    port: u8,
    child_speed: u8,
) -> Option<Tt> {
    if !matches!(child_speed, SPEED_FULL | SPEED_LOW) {
        return None;
    }
    if hub_speed == SPEED_HIGH {
        return Some(Tt { hub_slot, port });
    }
    hub_tt
}
