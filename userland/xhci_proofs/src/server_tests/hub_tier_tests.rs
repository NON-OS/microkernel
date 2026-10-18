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

//! Hubs below hubs: a full speed hub below a high-speed one hands that
//! hub's TT down to the slow devices below it, and the fifth tier is the
//! last a hub may sit at (USB 2.0 section 4.1.1).

use super::bench::Bench;
use super::hub_bench::{ADDRESS_DEVICE, CONFIGURE_ENDPOINT, FULL, HIGH, LOW};
use crate::protocol::E_INVAL;

#[test]
fn a_full_speed_hub_below_a_high_speed_hub_hands_its_tt_down() {
    let mut b = Bench::new(HIGH as u32);
    let top = b.root_device();
    assert_eq!(b.configure_hub(top, 4, 0, 0), 0);
    let fs_hub = b.enable_slot();
    assert_eq!(b.routed(fs_hub, FULL, 0x2, (top, 2)).status, 0);
    assert_eq!(b.configure_hub(fs_hub, 4, 3, 1), 0);
    let cfg = b.last(CONFIGURE_ENDPOINT);
    assert_eq!(cfg.input[2] >> 25 & 1, 0, "no MTT on a full speed hub");
    assert_eq!(cfg.input[4] >> 16 & 3, 0, "no think time on a full speed hub");
    let mouse = b.enable_slot();
    assert_eq!(b.routed(mouse, LOW, 0x12, (fs_hub, 1)).status, E_INVAL, "the hub has no TT");
    assert_eq!(b.routed(mouse, LOW, 0x12, (top, 2)).status, 0);
    let addr = b.last(ADDRESS_DEVICE);
    assert_eq!(addr.input[2] & 0xF_FFFF, 0x12);
    assert_eq!(addr.input[4] & 0xFFFF, top as u32 | 2 << 8, "the high-speed hub's TT");
}

#[test]
fn a_device_five_tiers_down_is_addressed_but_is_never_a_hub() {
    let mut b = Bench::new(HIGH as u32);
    let mut hub = b.root_device();
    assert_eq!(b.configure_hub(hub, 2, 0, 0), 0);
    let mut route = 0u32;
    for tier in 0..4 {
        route |= 1 << (4 * tier);
        hub = b.hub_below(HIGH, route, (0, 0), 2);
    }
    let deepest = b.enable_slot();
    route |= 1 << 16;
    assert_eq!(b.routed(deepest, HIGH, route, (0, 0)).status, 0, "five hubs above it");
    assert_eq!(b.configure_hub(deepest, 2, 0, 0), E_INVAL, "its ports would be a sixth tier");
    assert_ne!(hub, deepest);
}
