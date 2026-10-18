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
//! The core's own bit rule, beside the one every other driver uses.

use nonos_wifi_core::address::local_unicast;

#[test]
fn the_cores_rule_agrees_with_nonos_mac_on_every_first_octet() {
    for first in 0..=255u8 {
        let a = [first, 0x12, 0x34, 0x56, 0x78, 0x9a];
        let mut b = a;
        nonos_mac::apply(&mut b);
        assert_eq!(local_unicast(a), b, "first octet {first:#04x}");
        assert!(nonos_mac::is_local_unicast(&local_unicast(a)));
    }
}
