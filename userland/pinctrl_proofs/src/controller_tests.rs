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

//! The controller each `_HID` picks: the AMD bank by its ids (seven
//! characters with the trailing zero, or eight), and every Intel id
//! refused while no layout derived from a published datasheet exists.

use nonos_pinctrl::{controller, locate, Controller, PinError, LAYOUTS};

#[test]
fn amd_ids_and_unknown_ids() {
    for id in [b"AMD0030\0", b"AMDI0030", b"AMDI0031", b"AMDI0033"] {
        assert!(matches!(controller(id), Some(Controller::Amd)));
    }
    assert!(controller(b"AMD0030X").is_none(), "seven characters need the trailing zero");
    assert!(controller(b"AMDI0010").is_none(), "the AMD I2C controller is not GPIO");
    assert!(controller(b"\0\0\0\0\0\0\0\0").is_none());
}

#[test]
fn intel_controllers_are_refused_without_a_published_layout() {
    assert!(LAYOUTS.is_empty());
    for id in [b"INT344B\0", b"INT34BB\0", b"INTC1055", b"INTC105E"] {
        assert!(controller(id).is_none());
        assert_eq!(locate(id, 103), Err(PinError::UnknownController), "never guessed");
    }
    assert!(matches!(locate(b"AMDI0030", 9), Ok(nonos_pinctrl::Line::Amd { pin: 9 })));
}
