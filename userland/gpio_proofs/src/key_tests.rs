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

//! The one key form a capsule is given for a bus controller, read from the
//! end of a SERIAL record: present and marked, or refused.

use nonos_gpio::serial_key::serial_string;
use nonos_gpio::{bus_key, device_of, DEVICE};

/// A SERIAL record cut to what the key reads: the four strings at 42
/// (device path, `_HID`, bus controller path, GPIO path), then the key.
fn serial(bus_path: &[u8], key: u64) -> Vec<u8> {
    let mut r = vec![0u8; 42];
    for s in [&b"\\_SB.I2CA.TPD0"[..], b"PNP0C50", bus_path, b"\\_SB.GPIO"] {
        r.push(s.len() as u8);
        r.extend_from_slice(s);
    }
    r.extend_from_slice(&key.to_le_bytes());
    r
}

#[test]
fn the_key_after_the_strings_names_a_broker_device() {
    let r = serial(b"\\_SB.I2CA", DEVICE | 42);
    assert_eq!(bus_key(&r), Some(DEVICE | 42));
    assert_eq!(device_of(DEVICE | 42), Some(42));
    assert_eq!(serial_string(&r, 2), Some(&b"\\_SB.I2CA"[..]));
    let pci = serial(b"\\_SB.PCI0.I2C1", DEVICE | 7);
    assert_eq!(bus_key(&pci), Some(DEVICE | 7), "a PCI controller is keyed the same way");
}

#[test]
fn anything_but_that_form_is_refused() {
    assert_eq!(bus_key(&serial(b"\\_SB.I2CA", u64::MAX >> 1)), None, "bit 63 clear: a PCI form");
    assert_eq!(device_of(0x15 << 3 | 1), None);
    assert_eq!(bus_key(&serial(b"", 0x0A9)), None);
    let mut cut = serial(b"\\_SB.I2CA", DEVICE | 42);
    cut.truncate(cut.len() - 1);
    assert_eq!(bus_key(&cut), None, "key cut short");
    assert_eq!(bus_key(&[0; 20]), None);
}

#[test]
fn all_ones_names_no_controller() {
    // capsule_acpi writes all ones when no device in the table is the
    // controller: no pin is bound to it.
    assert_eq!(bus_key(&serial(b"\\_SB.I2CA", u64::MAX)), None);
}
