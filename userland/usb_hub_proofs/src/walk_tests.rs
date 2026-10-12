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
//! The hub walk over a modelled high-speed hub: attach, a device brought
//! up through its route and TT, unplugged, and a device nothing binds.

use nonos_usb_hub::{scan_hubs, Place, Tt, SPEED_FULL, SPEED_HIGH};

use crate::fake::{root_hub, Bind, Device, Fake, HubModel, Kind};

fn dev(speed: u8, kind: Kind) -> Device {
    Device { speed, kind }
}

#[test]
fn a_hub_is_told_to_the_controller_claimed_and_powered() {
    let mut h = Fake::default();
    let hub = root_hub(&mut h, HubModel::new(false, 4), 2, SPEED_HIGH);
    assert_eq!(h.configured, vec![(1, 4, 1)], "4 ports, think time 1");
    assert_eq!(h.claimed, vec![1], "its status endpoint");
    assert!(h.hubs[&1].ports.iter().all(|p| p.powered));
    assert!(h.now >= 100, "power-on time waited: {}", h.now);
    assert_eq!((hub.slot, hub.ports, hub.place.depth), (1, 4, 0));
    assert!(h.said("high speed hub in slot 1, tier 1, 4 ports"));
}

#[test]
fn a_full_speed_device_is_addressed_through_its_route_and_the_hubs_tt() {
    let mut h = Fake::default();
    let model = HubModel::new(false, 4).plug(3, dev(SPEED_FULL, Kind::Bind));
    let mut hubs = vec![root_hub(&mut h, model, 2, SPEED_HIGH)];
    let mut c = Bind::default();
    scan_hubs(&mut h, &mut hubs, &mut c);
    let tt = Some(Tt { hub_slot: 1, port: 3 });
    let want = Place { root_port: 2, route: 0x3, depth: 1, speed: SPEED_FULL, tt, parent: (1, 3) };
    assert_eq!(h.addressed, vec![(2, want)]);
    assert_eq!((c.bound.clone(), hubs[0].child[2]), (vec![2], 2));
    assert!(h.disabled.is_empty());
    assert!(!h.port(1, 3).c_connection && !h.port(1, 3).c_reset, "change bits cleared");
    // Unplugged: forgotten, its slot given back, the port empty again.
    h.unplug(1, 3);
    scan_hubs(&mut h, &mut hubs, &mut c);
    assert_eq!((c.forgotten, h.disabled.clone(), hubs[0].child[2]), (vec![2], vec![2], 0));
    assert!(h.said("device unplugged"));
}

#[test]
fn a_device_nothing_binds_is_given_back_and_tried_again_only_when_replugged() {
    let mut h = Fake::default();
    let model = HubModel::new(false, 2).plug(1, dev(SPEED_HIGH, Kind::Ignore));
    let mut hubs = vec![root_hub(&mut h, model, 1, SPEED_HIGH)];
    let mut c = Bind::default();
    scan_hubs(&mut h, &mut hubs, &mut c);
    scan_hubs(&mut h, &mut hubs, &mut c);
    assert_eq!((h.addressed.len(), h.disabled.clone()), (1, vec![2]), "once, then left");
    assert_eq!(h.addressed[0].1.tt, None, "no TT for a high-speed device");
    h.unplug(1, 1);
    scan_hubs(&mut h, &mut hubs, &mut c);
    h.port(1, 1).device = Some(dev(SPEED_HIGH, Kind::Bind));
    h.port(1, 1).c_connection = true;
    scan_hubs(&mut h, &mut hubs, &mut c);
    assert_eq!((h.addressed.len(), c.bound), (2, vec![3]), "the new device is looked at");
}
