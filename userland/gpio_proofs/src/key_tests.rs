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

//! Both forms of a pin's device key: a bus controller that is a PCI
//! function is named by it; one that is an ACPI platform device by its
//! broker device id with bit 63 set, found by its namespace path.

use nonos_gpio::device_key::pci_key;
use nonos_gpio::{bus_key, decode, Key, PLATFORM};

/// A SERIAL record cut to what the key reads: `_ADR` at 18, then the four
/// strings at 42 (device path, `_HID`, bus controller path, GPIO path).
fn serial(adr: u64, bus_path: &[u8]) -> Vec<u8> {
    let mut r = vec![0u8; 42];
    r[18..26].copy_from_slice(&adr.to_le_bytes());
    for s in [&b"\\_SB.I2CA.TPD0"[..], b"PNP0C50", bus_path, b"\\_SB.GPIO"] {
        r.push(s.len() as u8);
        r.extend_from_slice(s);
    }
    r
}

fn paths() -> Vec<(Vec<u8>, u64)> {
    vec![(b"\\_SB.GPIO".to_vec(), 7), (b"\\_SB.I2CA".to_vec(), 42)]
}

#[test]
fn a_pci_bus_controller_is_named_by_its_function() {
    // _ADR 0x0015_0001: device 0x15, function 1.
    let key = bus_key(&serial(0x0015_0001, b"\\_SB.PCI0.I2C1"), &paths()).unwrap();
    assert_eq!(key, 0x15 << 3 | 1);
    assert_eq!(key, pci_key(0x0015_0001));
    assert_eq!(decode(key), Key::Pci(0, 0, 0x15, 1));
    assert_eq!(decode(0x0002_0310), Key::Pci(2, 3, 2, 0), "segment and bus kept apart");
}

#[test]
fn a_platform_bus_controller_is_named_by_its_device_id() {
    let key = bus_key(&serial(u64::MAX, b"\\_SB.I2CA"), &paths()).unwrap();
    assert_eq!(key, PLATFORM | 42);
    assert_eq!(decode(key), Key::Device(42));
}

#[test]
fn a_bus_controller_that_is_neither_has_no_key() {
    assert_eq!(bus_key(&serial(u64::MAX, b"\\_SB.I2CB"), &paths()), None, "not published");
    assert_eq!(bus_key(&serial(u64::MAX, b""), &paths()), None, "no path");
    let wide = vec![(b"\\_SB.I2CA".to_vec(), PLATFORM | 1)];
    assert_eq!(bus_key(&serial(u64::MAX, b"\\_SB.I2CA"), &wide), None, "id uses bit 63");
    let mut cut = serial(u64::MAX, b"\\_SB.I2CA");
    cut.truncate(50);
    assert_eq!(bus_key(&cut, &paths()), None, "record cut short");
    assert_eq!(bus_key(&[0; 20], &paths()), None);
}
