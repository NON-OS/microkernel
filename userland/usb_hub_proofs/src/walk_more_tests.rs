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
//! The walk where it stops: a reset that never ends, an address the
//! controller driver refuses, and a hub below a hub taken away whole.

use nonos_usb_hub::{forget_root, Tt, E_INVAL, SPEED_HIGH, SPEED_LOW};

use crate::fake::{root_hub, walk, Bind, Device, Fake, HubModel, Kind};

#[test]
fn a_reset_that_never_ends_is_given_800_ms_three_times_a_minute() {
    let mut h = Fake::default();
    let model = HubModel::new(false, 1).plug(1, Device { speed: SPEED_HIGH, kind: Kind::Bind });
    let mut hubs = vec![root_hub(&mut h, model, 1, SPEED_HIGH)];
    h.port(1, 1).resetting = u8::MAX;
    let mut c = Bind::default();
    // Each walk takes a try to its end: well inside a minute, the port's
    // budget is three.
    for _ in 0..5 {
        walk(&mut h, &mut hubs, &mut c);
    }
    assert!(h.addressed.is_empty() && h.next_slot == 1, "no slot taken");
    let timed_out = h.lines.iter().filter(|l| l.contains("did not finish within 800 ms"));
    assert_eq!(timed_out.count(), 3, "three tries of 800 ms, then left for the minute");
    assert!(h.now < 60_000);
}

#[test]
fn an_address_the_controller_driver_refuses_gives_the_slot_back() {
    let mut h = Fake::default();
    let model = HubModel::new(false, 1).plug(1, Device { speed: SPEED_HIGH, kind: Kind::Bind });
    let mut hubs = vec![root_hub(&mut h, model, 1, SPEED_HIGH)];
    h.refuse_address = Some(E_INVAL);
    walk(&mut h, &mut hubs, &mut Bind::default());
    assert_eq!(h.disabled, vec![2]);
    assert!(h.said("op 0x20 refused"));
}

#[test]
fn a_hub_below_a_hub_is_walked_and_goes_with_its_root_port() {
    let mut h = Fake::default();
    let low = Device { speed: SPEED_LOW, kind: Kind::Bind };
    let inner = HubModel::new(false, 4).plug(2, low);
    let outer =
        HubModel::new(false, 4).plug(1, Device { speed: SPEED_HIGH, kind: Kind::Hub(inner) });
    let mut hubs = vec![root_hub(&mut h, outer, 3, SPEED_HIGH)];
    let mut c = Bind::default();
    walk(&mut h, &mut hubs, &mut c);
    assert_eq!(hubs.len(), 2, "the inner hub is held");
    walk(&mut h, &mut hubs, &mut c);
    let (slot, place) = h.addressed[1];
    assert_eq!((slot, place.route, place.depth), (3, 0x21, 2), "port 1, then port 2");
    assert_eq!(place.tt, Some(Tt { hub_slot: 2, port: 2 }), "the nearest high-speed hub's TT");
    assert_eq!(forget_root(&mut h, 3, &mut hubs, &mut c), 1);
    assert!(hubs.is_empty());
    let mut gone = h.disabled.clone();
    gone.sort();
    assert_eq!(gone, vec![1, 2, 3], "the hubs and the mouse below them");
    assert!(c.forgotten.contains(&3));
}
