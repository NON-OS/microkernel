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
//! OP_PARK's body and the virtual ports it fills.

use nonos_usb_virt::{parse_park, Virt, VPORTS, VPORT_BASE};

/// Slot 5, below hub slot 2's port 3, full speed, route 0x13, TT (1, 1).
pub(crate) const BODY: [u8; 12] = [5, 2, 3, 1, 0x13, 0, 0, 0, 1, 1, 64, 0];

fn with(i: usize, v: u8) -> [u8; 12] {
    let mut b = BODY;
    b[i] = v;
    b
}

#[test]
fn a_park_body_is_read_whole_and_refused_out_of_shape() {
    let p = parse_park(&BODY).expect("parked");
    assert_eq!((p.slot, p.parent, p.parent_port, p.speed, p.route), (5, 2, 3, 1, 0x13));
    assert_eq!((p.tt, p.mps0, p.connected, p.changed, p.claimant), ((1, 1), 64, true, true, None));
    for (i, v, why) in [
        (0, 0, "slot 0"),
        (1, 5, "its own parent"),
        (2, 0, "port 0"),
        (2, 16, "port 16"),
        (3, 0, "speed 0"),
        (3, 6, "speed 6"),
        (4, 0, "route 0"),
        (4, 0x03, "a gap in the route"),
        (9, 0, "a TT slot without its port"),
        (8, 5, "its own TT"),
        (10, 0, "EP0 size 0"),
    ] {
        let mut b = with(i, v);
        if i == 4 && v == 0x03 {
            b[5] = 1;
        }
        assert_eq!(parse_park(&b), None, "{why}");
    }
    assert_eq!(parse_park(&BODY[..11]), None, "short");
}

#[test]
fn ports_are_numbered_from_0x80_once_each_and_the_table_fills() {
    let mut v = Virt::default();
    let p = parse_park(&BODY).unwrap();
    assert_eq!(v.park(p), Some(VPORT_BASE));
    assert_eq!(v.park(p), None, "slot 5 is parked already");
    for s in 0..VPORTS as u8 - 1 {
        assert!(v.park(parse_park(&with(0, 10 + s)).unwrap()).is_some());
    }
    assert_eq!(v.park(parse_park(&with(0, 100)).unwrap()), None, "every port in use");
}
