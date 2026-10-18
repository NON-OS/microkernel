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

//! A device that is not a stick says its first interface's class, read by
//! the driver's own walk (scan/interface.rs) and said by the kernel's words.

use super::driver_report::{first_interface, Report, Stage};
use super::said;

#[test]
fn a_device_of_another_class_says_its_class() {
    /* A configuration descriptor whose first interface is a HID keyboard. */
    let config = [
        9, 2, 25, 0, 1, 1, 0, 0xa0, 50, //
        9, 4, 0, 0, 1, 3, 1, 1, 0, //
        7, 5, 0x81, 3, 8, 0, 10,
    ];
    let class = first_interface(&config);
    assert_eq!(class, 0x03_01_01);
    let line = said(Report { aux: class, ..Report::at(Stage::NotStorage, 2, 0) });
    assert!(line.contains("class 03, subclass 01, protocol 01"), "{line}");
}

#[test]
fn a_truncated_descriptor_names_no_class() {
    assert_eq!(first_interface(&[9, 2, 25]), 0);
    assert_eq!(first_interface(&[]), 0);
}
