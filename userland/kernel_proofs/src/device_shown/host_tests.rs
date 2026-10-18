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

use super::host_id::{host_device_id, UNRESOLVED};
use super::tests::{HOST_BASE, HOST_ID};

#[test]
fn the_hid_record_names_its_host_by_device_id() {
    let hosts = [(*b"I2C0", 0xFE02_0000), (*b"I2C1", HOST_BASE)];
    let registered = [(0xFE02_0000, 0x1_0000_0002), (HOST_BASE, HOST_ID)];
    assert_eq!(host_device_id(*b"I2C1", hosts, &registered), HOST_ID);
    assert_eq!(host_device_id(*b"I2C0", hosts, &registered), 0x1_0000_0002);
}

#[test]
fn a_host_not_found_is_unresolved_never_an_address() {
    let hosts = [(*b"I2C1", HOST_BASE), (*b"I2C2", 0)];
    let registered = [(HOST_BASE, HOST_ID), (0, 0x1_0000_0009)];
    assert_eq!(host_device_id(*b"I2C3", hosts, &registered), UNRESOLVED, "no such name");
    assert_eq!(host_device_id(*b"I2C1", hosts, &[]), UNRESOLVED, "host not registered");
    assert_eq!(host_device_id(*b"I2C2", hosts, &registered), UNRESOLVED, "a zero base");
    assert_eq!(host_device_id([0; 4], [([0; 4], HOST_BASE)], &registered), UNRESOLVED);
}
