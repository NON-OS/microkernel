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

use alloc::format;

use super::say::{line, name};
use crate::discover::{find_touchpad_addrs, AcpiTouchpad, MAX_TARGETS};

/// The machine declares a touchpad on I2C but has no I2C controller this
/// driver supports: say so in words instead of leaving the pointer silently
/// dead. Silent when nothing is declared (a desktop, or a PS/2 or USB pad).
pub fn say_no_controller() {
    let mut buf = [AcpiTouchpad::default(); MAX_TARGETS];
    let t = find_touchpad_addrs(&mut buf);
    if let Some(tg) = buf[..t].first() {
        line(format!(
            "driver.i2c_pci: the firmware declares a touchpad on I2C controller {}, but this machine's I2C controller is not one NONOS supports: no touchpad pointer\n",
            name(&tg.host_name)
        ));
    }
}
