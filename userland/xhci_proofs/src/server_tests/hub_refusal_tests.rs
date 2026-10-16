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

//! Every field of OP_ADDRESS_ROUTED is the client's. One that names no
//! place this driver set up is refused before any command reaches the
//! controller: a route with no tier, a gap, or bits past the fifth; a port
//! past the hub's count; a parent that is no hub; another root port; a
//! speed the hub's ports do not carry; a TT the hub's place does not
//! imply; a slot not enabled or already addressed.

use super::bench::Bench;
use super::hub_bench::{routed_body as body, Routed, HIGH, LOW, SUPER};
use crate::protocol::{E_BUSY, E_INVAL, OP_ADDRESS_ROUTED};

#[test]
fn a_routed_address_naming_no_place_of_a_configured_hub_is_refused() {
    let mut b = Bench::new(HIGH as u32);
    let hub = b.root_device();
    assert_eq!(b.configure_hub(hub, 4, 0, 0), 0);
    let s = b.enable_slot();
    let ran = b.ran().len();
    let hostile: [Routed; 20] = [
        (s, 1, LOW, 0, hub, 3),
        (s, 1, LOW, 0x30, hub, 3),
        (s, 1, LOW, 0x3 | 1 << 20, hub, 3),
        (s, 1, LOW, 0x5, hub, 5),
        (s, 1, LOW, 0xF, hub, 15),
        (s, 1, LOW, 0x13, hub, 3),
        (s, 2, LOW, 0x3, hub, 3),
        (s, 0, LOW, 0x3, hub, 3),
        (s, 255, LOW, 0x3, hub, 3),
        (s, 1, SUPER, 0x3, 0, 0),
        (s, 1, 0, 0x3, 0, 0),
        (s, 1, 0xFF, 0x3, 0, 0),
        (s, 1, LOW, 0x3, 0, 0),
        (s, 1, LOW, 0x3, hub, 2),
        (s, 1, LOW, 0x3, s, 3),
        (s, 1, HIGH, 0x3, hub, 3),
        (0, 1, LOW, 0x3, hub, 3),
        (7, 1, LOW, 0x3, hub, 3),
        (255, 1, LOW, 0x3, hub, 3),
        (hub, 1, LOW, 0x3, hub, 3),
    ];
    for h in hostile {
        assert_eq!(b.call(OP_ADDRESS_ROUTED, &body(h)).status, E_INVAL, "{h:x?}");
    }
    let good = body((s, 1, LOW, 0x3, hub, 3));
    for len in [0, 8, 10] {
        let mut wrong = good.clone();
        wrong.resize(len, 0);
        assert_eq!(b.call(OP_ADDRESS_ROUTED, &wrong).status, E_INVAL, "{len} bytes");
    }
    assert_eq!(b.ran().len(), ran, "no command reached the controller");
    assert_eq!(b.call(OP_ADDRESS_ROUTED, &good).status, 0, "the place itself is fine");
    let twin = b.enable_slot();
    let again = body((twin, 1, LOW, 0x3, hub, 3));
    assert_eq!(b.call(OP_ADDRESS_ROUTED, &again).status, E_BUSY, "one device per hub port");
}
