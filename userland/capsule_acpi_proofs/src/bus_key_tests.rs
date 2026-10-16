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

//! A bus controller named either way firmware can name it (its `_ADR`
//! resolved to a PCI function, or its namespace path) comes out as the
//! same one key, `1 << 63 | broker device id`; nothing else is a key.

use crate::bus_key::{device_of, key, Controller, Known, DEVICE};

fn table() -> Vec<Known> {
    vec![
        Known { device_id: 3, bdf: Some((0, 0, 0x14, 0)), path: Vec::new() },
        // An LPSS I2C function the namespace also describes by path.
        Known { device_id: 5, bdf: Some((0, 0, 0x15, 1)), path: b"\\_SB.PCI0.I2C1".to_vec() },
        // An AMD platform controller: a path, no function.
        Known { device_id: 9, bdf: None, path: b"\\_SB.I2CA".to_vec() },
    ]
}

#[test]
fn both_forms_of_one_controller_are_one_key() {
    let t = table();
    let by_function = key(Controller::Pci((0, 0, 0x15, 1)), &t);
    let by_path = key(Controller::Path(b"\\_SB.PCI0.I2C1"), &t);
    assert_eq!(by_function, Some(DEVICE | 5));
    assert_eq!(by_function, by_path, "the PCI form and the path name the same controller");
    assert_eq!(device_of(by_function.unwrap()), Some(5));
    assert_eq!(key(Controller::Path(b"\\_SB.I2CA"), &t), Some(DEVICE | 9));
}

#[test]
fn a_controller_not_in_the_table_has_no_key() {
    let t = table();
    assert_eq!(key(Controller::Pci((0, 0, 0x15, 2)), &t), None);
    assert_eq!(key(Controller::Path(b"\\_SB.I2CB"), &t), None);
    assert_eq!(key(Controller::Path(b""), &t), None, "an empty path matches nothing");
    let wide = [Known { device_id: DEVICE | 1, bdf: Some((0, 0, 1, 0)), path: Vec::new() }];
    assert_eq!(key(Controller::Pci((0, 0, 1, 0)), &wide), None, "id already uses the bit");
}

#[test]
fn the_pci_form_is_not_a_key() {
    assert_eq!(device_of(0x15 << 3 | 1), None);
    assert_eq!(device_of(DEVICE | 7), Some(7));
}
