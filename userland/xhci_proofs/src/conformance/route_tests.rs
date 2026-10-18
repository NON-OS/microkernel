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

//! The route string and the Slot Context fields a device below a hub needs
//! (xHCI 1.2 sections 6.2.2 and 8.9), against the bit positions the
//! specification gives, for every edge of each field.

use crate::contexts::slot::slot_dwords;
use crate::contexts::{parent_of, route_depth, Place, Topology};

#[test]
fn a_route_has_up_to_five_tiers_with_no_gap() {
    assert_eq!(route_depth(0), Some(0), "on the root port");
    assert_eq!(route_depth(0x3), Some(1));
    assert_eq!(route_depth(0x12345), Some(5));
    assert_eq!(route_depth(0xF_FFFF), Some(5));
    assert_eq!(route_depth(0x10_0000), None, "a sixth tier");
    assert_eq!(route_depth(0x1F_FFFF), None);
    assert_eq!(route_depth(u32::MAX), None);
    assert_eq!(route_depth(0x30), None, "an empty tier below one in use");
    assert_eq!(route_depth(0x1_0003), None);
}

#[test]
fn a_device_hangs_off_the_route_less_its_last_tier() {
    assert_eq!(parent_of(0x3, 1), (0, 3));
    assert_eq!(parent_of(0x23, 2), (0x3, 2));
    assert_eq!(parent_of(0xF_4321, 5), (0x4321, 15));
    // Depths outside 1 to 5 are clamped, not shifted past the word.
    assert_eq!(parent_of(0x3, 0), (0, 3));
    assert_eq!(parent_of(u32::MAX, 255), (0xFFFF, 15));
}

#[test]
fn the_slot_context_carries_route_speed_root_port_and_tt_in_their_fields() {
    let topo = Topology { route: 0xF_4321, tt_slot: 9, tt_port: 4, mtt: true };
    let [dw0, dw1, dw2] = slot_dwords(&Place { root_port: 7, speed: 2, topo });
    assert_eq!(dw0, 0xF_4321 | 2 << 20 | 1 << 25 | 1 << 27, "route, speed, MTT, one entry");
    assert_eq!(dw1, 7 << 16, "root hub port number");
    assert_eq!(dw2, 9 | 4 << 8, "TT hub slot and port, no think time");
}

#[test]
fn no_field_spills_into_its_neighbour() {
    let topo = Topology { route: u32::MAX, tt_slot: 255, tt_port: 255, mtt: false };
    let [dw0, dw1, dw2] = slot_dwords(&Place { root_port: 255, speed: 0xFF, topo });
    assert_eq!(dw0, 0xF_FFFF | 0xF << 20 | 1 << 27, "route and speed cut to their bits");
    assert_eq!(dw1, 0xFF << 16);
    assert_eq!(dw2, 0xFFFF, "nothing in TT Think Time or the interrupter target");
    let no_tt = Topology { mtt: true, ..Topology::default() };
    let [dw0, ..] = slot_dwords(&Place { root_port: 1, speed: 3, topo: no_tt });
    assert_eq!(dw0 & 1 << 25, 0, "MTT only with a TT to name");
}
