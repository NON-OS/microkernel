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

use crate::vector_tests::layout;
use nonos_pinctrl::{locate, resolve, Line, Pad, PinError};

#[test]
fn the_same_number_is_a_different_pad_on_another_platform() {
    // Firmware pin 103 is GPP_E7 on Sunrise Point but GPP_D7 on Cannon
    // Point: reading the number as a pad index only works on the first.
    assert_eq!(resolve(layout("Sunrise Point-LP"), 103), Ok(Pad { bar: 1, pad: 55 }));
    assert_eq!(resolve(layout("Cannon Point-LP"), 103), Ok(Pad { bar: 1, pad: 7 }));
}

#[test]
fn unnumbered_groups_and_gaps_are_refused() {
    // Cannon Point-LP numbers GPP_B 32..57 and GPP_G from 64; 58..63 is
    // nothing, and its SPI pads have no firmware number at all.
    assert_eq!(resolve(layout("Cannon Point-LP"), 60), Err(PinError::NotMapped));
    assert_eq!(resolve(layout("Tiger Lake-LP"), 1000), Err(PinError::NotMapped));
}

#[test]
fn a_zero_based_group_resolves_from_zero() {
    // Jasper Lake GPP_G is INTEL_GPIO_BASE_ZERO: firmware pin 3 is pin 228.
    assert_eq!(resolve(layout("Jasper Lake"), 3), Ok(Pad { bar: 3, pad: 3 }));
}

#[test]
fn locate_by_hid() {
    let tgl = *b"INTC1055";
    assert_eq!(locate(&tgl, 327), Ok(Line::Intel { bar: 2, pad: 62 }));
    assert_eq!(locate(b"AMDI0030", 9), Ok(Line::Amd { pin: 9 }));
    assert_eq!(locate(b"INT3452\0", 9), Err(PinError::UnknownController));
}
