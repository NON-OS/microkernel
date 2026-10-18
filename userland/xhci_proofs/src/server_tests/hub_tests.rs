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

//! A device below a hub, end to end through the server: the hub on root
//! port 1 is addressed and configured as a hub (Hub, Number of Ports, TTT
//! and MTT in its Slot Context, xHCI 1.2 section 6.2.2), and the device on
//! one of its ports is addressed with its route string, the root port, its
//! speed, and the TT a low or full speed device needs.

use super::bench::Bench;
use super::hub_bench::{ADDRESS_DEVICE, CONFIGURE_ENDPOINT, HIGH, LOW, SUPER};

const HUB_BIT: u32 = 1 << 26;
const MTT_BIT: u32 = 1 << 25;

#[test]
fn a_low_speed_device_below_a_high_speed_hub_is_addressed_with_its_route_and_tt() {
    let mut b = Bench::new(HIGH as u32);
    let hub = b.root_device();
    assert_eq!(b.configure_hub(hub, 4, 2, 1), 0);
    let cfg = b.last(CONFIGURE_ENDPOINT);
    assert_eq!(cfg.trb.slot_id(), hub);
    assert_eq!(cfg.input[0..2], [0, 1], "only A0: the slot context, no endpoint");
    assert_eq!(cfg.input[2] & (HUB_BIT | MTT_BIT), HUB_BIT | MTT_BIT, "Hub, and MTT");
    assert_eq!(cfg.input[3] >> 24, 4, "Number of Ports");
    assert_eq!(cfg.input[4] >> 16 & 3, 2, "TT Think Time");
    let child = b.enable_slot();
    let r = b.routed(child, LOW, 0x3, (hub, 3));
    assert_eq!(r.status, 0);
    assert_eq!(r.body[..6], [child, 1, LOW, 0, 8, 0], "slot, root port, speed, EP0 size");
    let addr = b.last(ADDRESS_DEVICE);
    assert_eq!(addr.trb.slot_id(), child);
    assert_eq!(addr.input[2] & 0xF_FFFF, 0x3, "route string");
    assert_eq!(addr.input[2] >> 20 & 0xF, LOW as u32, "speed");
    assert_ne!(addr.input[2] & MTT_BIT, 0, "its TT hub runs a TT per port");
    assert_eq!(addr.input[3] >> 16 & 0xFF, 1, "root hub port");
    assert_eq!(addr.input[4] & 0xFFFF, hub as u32 | 3 << 8, "TT hub slot and port");
}

#[test]
fn a_superspeed_device_below_a_superspeed_hub_has_no_tt() {
    let mut b = Bench::new(SUPER as u32);
    let hub = b.root_device();
    assert_eq!(b.configure_hub(hub, 7, 0, 0), 0);
    let cfg = b.last(CONFIGURE_ENDPOINT);
    assert_eq!(cfg.input[2] & (HUB_BIT | MTT_BIT), HUB_BIT, "Hub, never MTT");
    assert_eq!(cfg.input[3] >> 24, 7);
    assert_eq!(cfg.input[4] >> 16 & 3, 0, "no TT, so no think time");
    let child = b.enable_slot();
    assert_eq!(b.routed(child, SUPER, 0x7, (0, 0)).status, 0);
    let addr = b.last(ADDRESS_DEVICE);
    assert_eq!(addr.input[2] & 0xF_FFFF, 0x7);
    assert_eq!(addr.input[4] & 0xFFFF, 0, "no TT");
}
