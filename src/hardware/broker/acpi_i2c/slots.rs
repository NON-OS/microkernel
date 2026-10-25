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

//! The NameSeg encodings an I2C-HID record carries.

use crate::arch::firmware_table::devices::GpioController;

/// Resolve the GpioInt ResourceSource NameSeg against the enumerated GPIO
/// community controllers and encode the match as `_UID`+1, zero when the
/// touchpad named no community, the name resolves to no enumerated controller,
/// or the index does not fit the field. The input driver uses it to select
/// which registered GPIO_CTRL record (matched on `device` == `_UID`) carries
/// the pad's interrupt pad.
pub(super) fn gpio_community_slot(name: [u8; 4], gpio: &[GpioController]) -> u8 {
    if name == [0u8; 4] {
        return 0;
    }
    for ctl in gpio {
        if ctl.name == name {
            return match ctl.uid.checked_add(1) {
                Some(slot) if slot <= u32::from(u8::MAX) => slot as u8,
                _ => 0,
            };
        }
    }
    0
}

/// Encode the controller NameSeg "I2C0".."I2C9" as index+1, zero when absent
/// or not of that shape. Intel firmware names its LPSS I2C functions this way
/// in PCI function order, which is what lets the host driver map the name to
/// a PCI device id.
pub(super) fn controller_slot(name: [u8; 4]) -> u8 {
    if name[0..3] == *b"I2C" && name[3].is_ascii_digit() {
        name[3] - b'0' + 1
    } else {
        0
    }
}
