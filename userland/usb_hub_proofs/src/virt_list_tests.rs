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
//! OP_PARK's body and the virtual ports it fills, and the bytes a host
//! The bytes a host lists a virtual port with in PORT_STATUS.

use nonos_usb_virt::{ext, parse_park, port_entry, Virt};

use crate::virt_park_tests::BODY;

#[test]
fn the_listing_bytes_are_exact_and_a_change_is_told_once() {
    let mut v = Virt::default();
    let vp = v.park(parse_park(&BODY).unwrap()).unwrap();
    let p = *v.at(vp).unwrap();
    // Connected, enabled, full speed (1 << 10), connect change (1 << 17).
    assert_eq!(port_entry(vp, &p, 2), [0x80, 0, 1, 0, 0x03, 0x04, 0x02, 0x00]);
    assert_eq!(ext(vp, &p), [0x80, 2, 1, 1, 0x13, 0, 0, 0]);
    v.reported();
    assert_eq!(port_entry(vp, v.at(vp).unwrap(), 2)[6], 0, "the change went with the listing");
    v.claim(vp, 9).unwrap();
    assert_eq!(port_entry(vp, v.at(vp).unwrap(), 2)[1], 2, "claimed: the host's owner byte");
    assert_eq!(port_entry(vp, v.at(vp).unwrap(), 0)[1], 1, "at least addressed");
    assert_eq!(v.entries().map(|(n, _)| n).collect::<Vec<_>>(), vec![0x80]);
}
