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
//! driver.usb_hub0's classes (nonos_usb_hub's Parker) over the walk: a
//! hub below a hub configured and walked, every other child parked with
//! the host by OP_PARK, whose body nonos_usb_virt reads back whole.

use nonos_usb_hub::{park_body, Parker, Place, Tt, SPEED_HIGH, SPEED_LOW};
use nonos_usb_virt::parse_park;

use crate::fake::{root_hub, walk, Device, Fake, HubModel, Kind};

#[test]
fn a_park_body_reads_back_as_written() {
    let tt = Some(Tt { hub_slot: 2, port: 4 });
    let place = Place { root_port: 1, route: 0x41, depth: 2, speed: 2, tt, parent: (3, 4) };
    let p = parse_park(&park_body(7, &place, 8)).expect("a body the host takes");
    assert_eq!((p.slot, p.parent, p.parent_port, p.speed), (7, 3, 4, 2));
    assert_eq!((p.route, p.tt, p.mps0), (0x41, (2, 4), 8));
}

#[test]
fn children_are_parked_and_a_hub_below_is_walked() {
    let mut h = Fake::default();
    let low = Device { speed: SPEED_LOW, kind: Kind::Bind };
    let inner = HubModel::new(false, 4).plug(3, low);
    let stick = Device { speed: SPEED_HIGH, kind: Kind::Ignore };
    let outer = HubModel::new(false, 4)
        .plug(1, Device { speed: SPEED_HIGH, kind: Kind::Hub(inner) })
        .plug(2, stick);
    let mut hubs = vec![root_hub(&mut h, outer, 1, SPEED_HIGH)];
    walk(&mut h, &mut hubs, &mut Parker);
    walk(&mut h, &mut hubs, &mut Parker);
    assert_eq!(hubs.len(), 2, "the inner hub is held");
    let parked: Vec<(u8, u32, (u8, u8), u16)> =
        h.parked.iter().map(|(s, p, m)| (*s, p.route, p.parent, *m)).collect();
    assert_eq!(parked, vec![(3, 0x2, (1, 2), 64), (4, 0x31, (2, 3), 8)]);
    assert!(h.disabled.is_empty(), "parked children keep their slots");
}
