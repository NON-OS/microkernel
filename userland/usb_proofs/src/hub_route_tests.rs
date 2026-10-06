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

//! The route string and TT rules of xHCI 1.2 section 8.9 and 6.2.2.

use crate::hub::port_status::{SPEED_FULL, SPEED_HIGH, SPEED_LOW, SPEED_SUPER};
use crate::hub::route::{child_route, child_tt, Tt};

#[test]
fn route_strings_put_one_port_per_tier_from_the_lowest_nibble() {
    assert_eq!(child_route(0, 0, 3), Some(0x3));
    assert_eq!(child_route(0x3, 1, 2), Some(0x23));
    assert_eq!(child_route(0x23, 2, 15), Some(0xf23));
    assert_eq!(child_route(0, 0, 28), Some(0xf));
    assert_eq!(child_route(0x4321, 4, 1), Some(0x14321));
    assert_eq!(child_route(0x14321, 5, 1), None);
    assert_eq!(child_route(0, 0, 0), None);
}

#[test]
fn only_slow_devices_get_a_tt_from_the_nearest_high_speed_hub() {
    let tt = Tt { hub_slot: 4, port: 2 };
    assert_eq!(child_tt(SPEED_HIGH, 4, None, 2, SPEED_LOW), Some(tt));
    assert_eq!(child_tt(SPEED_HIGH, 4, None, 2, SPEED_FULL), Some(tt));
    assert_eq!(child_tt(SPEED_HIGH, 4, None, 2, SPEED_HIGH), None);
    assert_eq!(child_tt(SPEED_SUPER, 4, None, 2, SPEED_SUPER), None);
    // A full speed hub below a high-speed one hands down that hub's TT.
    assert_eq!(child_tt(SPEED_FULL, 7, Some(tt), 1, SPEED_LOW), Some(tt));
    // A full speed hub on a root port: the root port itself translates.
    assert_eq!(child_tt(SPEED_FULL, 7, None, 1, SPEED_LOW), None);
}
