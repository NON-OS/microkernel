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

//! A touchpad under a PCI LPSS host resolves to that function's device_id
//!: Gemini Lake's I2C hosts are 8086:31AC.. at 00:16.x and 00:17.x.

use super::host_id::{pci_host_device_id, UNRESOLVED};

const GLK_I2C7: u64 = 0x31AC_0007;
const SKL_I2C0: u64 = 0x9D60_0000;

/// The enumerated segment 0 functions: GLK's I2C7 at 00:17.3 and an
/// I2C0 at 00:15.0, beside the host bridge.
fn functions() -> [((u8, u8, u8), u64); 3] {
    [((0, 0, 0), 1), ((0, 0x17, 3), GLK_I2C7), ((0, 0x15, 0), SKL_I2C0)]
}

#[test]
fn an_lpss_host_resolves_by_its_adr_to_the_enumerated_function() {
    let hosts = [(*b"I2C0", 0x0015_0000), (*b"I2C7", 0x0017_0003), (*b"XHC_", 0x0014_0000)];
    assert_eq!(pci_host_device_id(*b"I2C7", hosts, &functions()), GLK_I2C7);
    assert_eq!(pci_host_device_id(*b"I2C0", hosts, &functions()), SKL_I2C0);
}

#[test]
fn a_name_the_function_list_cannot_settle_is_unresolved() {
    let f = functions();
    let none: [([u8; 4], u32); 0] = [];
    assert_eq!(pci_host_device_id(*b"I2C7", none, &f), UNRESOLVED, "no such name");
    let twice = [(*b"I2C7", 0x0017_0003), (*b"I2C7", 0x0016_0000)];
    assert_eq!(pci_host_device_id(*b"I2C7", twice, &f), UNRESOLVED, "two addresses");
    let same = [(*b"I2C7", 0x0017_0003), (*b"I2C7", 0x0017_0003)];
    assert_eq!(pci_host_device_id(*b"I2C7", same, &f), GLK_I2C7, "one address twice");
    let all = [(*b"I2C7", 0x0017_FFFF)];
    assert_eq!(pci_host_device_id(*b"I2C7", all, &f), UNRESOLVED, "every function");
    let absent = [(*b"I2C7", 0x0019_0002)];
    assert_eq!(pci_host_device_id(*b"I2C7", absent, &f), UNRESOLVED, "not enumerated");
    let wide = [(*b"I2C7", 0x0020_0000)];
    assert_eq!(pci_host_device_id(*b"I2C7", wide, &f), UNRESOLVED, "no device 32");
    assert_eq!(pci_host_device_id([0; 4], [([0; 4], 0x0017_0003)], &f), UNRESOLVED);
}

#[test]
fn only_the_root_bus_is_searched() {
    let f = [((1, 0x17, 3), GLK_I2C7)];
    assert_eq!(pci_host_device_id(*b"I2C7", [(*b"I2C7", 0x0017_0003)], &f), UNRESOLVED);
}
