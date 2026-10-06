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

//! Plain-language console lines about the touchpad bind, so a machine with no
//! serial port still tells its owner, on the boot console, why there is or
//! is no touchpad pointer.

use alloc::format;
use alloc::string::String;

use nonos_libc::mk_debug;

use crate::constants::{controller_index, HID_INFO_GPIO};
use crate::discover::{AcpiTouchpad, Found};
use crate::driver::Driver;

fn line(s: String) {
    let _ = mk_debug(s.as_ptr(), s.len());
}

fn name(seg: &[u8; 4]) -> String {
    seg.iter().take_while(|&&b| b != 0).map(|&b| b as char).collect()
}

pub(super) fn say_bound(driver: &Driver, tg: Option<&AcpiTouchpad>) {
    let pacing = if driver.doorbell.is_some() {
        "reads paced by its interrupt line"
    } else if tg.is_some_and(|t| t.info & HID_INFO_GPIO != 0) {
        "interrupt line on a GPIO layout NONOS does not map, reads are polled"
    } else {
        "reads are polled"
    };
    line(format!(
        "driver.i2c_pci: touchpad bus is {} {:04x} (I2C{}) at {} kHz, device 0x{:02x} {}, {}\n",
        driver.family,
        driver.pci_device,
        controller_index(driver.pci_device).map_or(String::from("?"), |i| format!("{}", i)),
        driver.clock_hz / 1000,
        driver.bound_addr,
        if driver.bound_by_probe { "answered" } else { "not answering yet" },
        pacing
    ));
}

pub(super) fn say_unreached(tg: &AcpiTouchpad, cands: &[Found]) {
    line(format!(
        "driver.i2c_pci: the firmware declares a touchpad at 0x{:02x} on I2C controller {}, but none of the {} I2C controllers NONOS can drive answers it or matches that name; the touchpad may not work\n",
        tg.addr,
        name(&tg.host_name),
        cands.len()
    ));
}

/// The machine declares a touchpad on I2C but has no I2C controller this
/// driver supports: say so in words instead of leaving the pointer silently
/// dead. Silent when nothing is declared (a desktop, or a PS/2 or USB pad).
pub fn say_no_controller() {
    let mut buf = [AcpiTouchpad::default(); crate::discover::MAX_TARGETS];
    let t = crate::discover::find_touchpad_addrs(&mut buf);
    if let Some(tg) = buf[..t].first() {
        line(format!(
            "driver.i2c_pci: the firmware declares a touchpad on I2C controller {}, but this machine's I2C controller is not one NONOS supports: no touchpad pointer\n",
            name(&tg.host_name)
        ));
    }
}
