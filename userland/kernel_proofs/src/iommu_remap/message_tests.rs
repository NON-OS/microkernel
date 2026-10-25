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

use super::msi::{handle, remapped};

/* VT-d 5.1.5.2: 0xFEE base, interrupt format bit 4, SHV bit 3, handle
14:0 in bits 19:5 and handle bit 15 in bit 2, data the subhandle 0. */
#[test]
fn the_message_names_the_entry() {
    assert_eq!(remapped(0).address, 0xFEE0_0018);
    assert_eq!(remapped(1).address, 0xFEE0_0038);
    assert_eq!(remapped(0x7FFF).address, 0xFEE0_0000 | (0x7FFF << 5) | 0x18);
    assert_eq!(remapped(0x8000).address, 0xFEE0_0000 | 0x18 | 0x4);
    assert_eq!(remapped(5).data, 0);
}

#[test]
fn every_handle_reads_back_and_compat_addresses_are_told_apart() {
    for i in 0..=u16::MAX {
        let address = remapped(i).address;
        assert_eq!(address & 0xFFF0_0000, 0xFEE0_0000);
        assert_eq!(handle(address), Some(i));
    }
    // A compatibility format address for APIC id 2: bit 4 clear.
    assert_eq!(handle(0xFEE0_2000), None);
}
