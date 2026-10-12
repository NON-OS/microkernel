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
//! Whether a routed address request may be served: the slot enabled and
//! not addressed, a hub on the root port, a route of one hub port per
//! tier, and a TT exactly when the device is slower than high speed, in
//! a high-speed hub's slot. Pure over the table.

use crate::devices::table::Table;
use crate::sched::link::Link;

pub struct Below {
    pub slot: u8,
    pub port: u8,
    pub route: u32,
    pub link: Link,
}

/// One 4-bit hub port per tier from the lowest, at most five, none after
/// the first empty one.
pub fn route_ok(route: u32) -> bool {
    let tiers = (0..5).take_while(|t| (route >> (4 * t)) & 0xF != 0).count();
    tiers > 0 && route >> (4 * tiers as u32) == 0
}

pub fn below(t: &Table, body: &[u8]) -> Option<Below> {
    let &[slot, port, speed @ 1..=3, r0, r1, r2, r3, tt_slot, tt_port] = body else { return None };
    let route = u32::from_le_bytes([r0, r1, r2, r3]);
    let fresh = t.devs.get((slot as usize).wrapping_sub(1))?.is_some_and(|d| d.port == 0);
    let tt = match (speed, tt_slot) {
        (3, 0) => None,
        (1 | 2, s) if t.tt_hub(s) && (1..=15).contains(&tt_port) => Some((s, tt_port)),
        _ => return None,
    };
    let ok = fresh && (1..=t.ports).contains(&port) && t.hub_on(port) && route_ok(route);
    ok.then_some(Below { slot, port, route, link: Link { speed, tt } })
}
