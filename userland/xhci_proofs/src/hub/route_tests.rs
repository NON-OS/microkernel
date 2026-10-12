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
//! A device below a hub, in Slot Context fields: which routes and TTs the
//! controller driver takes, and where each field lands in its dword.

use crate::contexts::route::{as_hub, route_ok, routed_fits, with_route, with_tt};

#[test]
fn a_route_is_one_hub_port_per_tier_from_the_lowest_up_to_five() {
    for good in [0x1, 0xF, 0x21, 0x4321, 0xF_FFFF, 0x1_1111] {
        assert!(route_ok(good), "{good:#x}");
    }
    // 0: the root port itself; a gap below a tier; a sixth tier.
    for bad in [0, 0x10, 0x101, 0x1_0001, 0x10_0000, 0x11_1111, u32::MAX] {
        assert!(!route_ok(bad), "{bad:#x}");
    }
}

#[test]
fn a_tt_is_a_hubs_port_and_serves_only_full_and_low_speed() {
    assert!(routed_fits(3, 0x1, None, false), "high speed, no TT");
    assert!(routed_fits(1, 0x2, Some((4, 2)), true), "full speed through hub 4 port 2");
    assert!(routed_fits(2, 0x31, Some((5, 3)), true), "low speed, two tiers down");
    assert!(routed_fits(1, 0x1, None, false), "full speed below a full speed hub");
    assert!(!routed_fits(3, 0x1, Some((4, 1)), true), "a TT for high speed");
    assert!(!routed_fits(4, 0x1, Some((4, 1)), true), "a TT for SuperSpeed");
    assert!(!routed_fits(1, 0x1, Some((4, 1)), false), "a TT in no hub's slot");
    for port in [0, 16, 255] {
        assert!(!routed_fits(1, 0x1, Some((4, port)), true), "TT port {port}");
    }
    for speed in [0, 6, 15] {
        assert!(!routed_fits(speed, 0x1, None, false), "speed {speed}");
    }
    assert!(!routed_fits(3, 0, None, false), "route 0 is the root port");
}

#[test]
fn each_field_lands_in_its_bits_and_nothing_else_moves() {
    let dw0 = 3 << 20 | 1 << 27;
    assert_eq!(with_route(dw0 | 0xF_FFFF, 0x21), dw0 | 0x21);
    let dw2 = 0x3 << 22 | 0x2 << 16;
    assert_eq!(with_tt(dw2 | 0xFFFF, 4, 2), dw2 | 4 | 2 << 8, "target and think time kept");
    let [a, b, c] = as_hub([dw0 | 0x21, 0x0005_0000, 0x0040_0000], 7, 2, false);
    assert_eq!(a, dw0 | 0x21 | 1 << 26, "Hub set, route and entries kept");
    assert_eq!(b, 7 << 24 | 0x0005_0000, "Number of Ports over the root port");
    assert_eq!(c, 0x0040_0000 | 2 << 16, "think time");
    let [a, ..] = as_hub([a, b, c], 7, 7, true);
    assert_eq!(a & (1 << 25), 1 << 25, "MTT when asked");
    assert_eq!(as_hub([a, b, c], 7, 7, false)[2] >> 16 & 0x3, 3, "think time is two bits");
}
