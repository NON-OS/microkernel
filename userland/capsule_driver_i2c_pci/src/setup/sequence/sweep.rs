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

//! The pad behind a declaration with no address. Intel's reference firmware
//! declares its pad as TPD0 with I2cSerialBusV2(0x0000, ...) and patches
//! the address in from NVS in _INI, which the kernel cannot run: the record
//! names the controller, no more. On that controller alone, the three
//! addresses touchpads commonly ship at are probed in turn with a plain
//! one-byte read, which writes nothing to whatever answers, not even a
//! register pointer, and the sweep ends at the first device that answers.
//! That device is bound whether it is a pad or not, and the HID driver
//! writes and reads it on every probe, so the list holds no address where
//! a board puts anything else: GPIO expanders (0x20-0x27), thermal sensors
//! (0x48-0x4F), whose configuration register an HID register pointer would
//! write, and touchscreens, which declare their address. Once the kernel
//! evaluates _CRS (K13 in the plan), the record carries
//! the address itself and the sweep goes.

use crate::constants::HID_INFO_TEN_BIT;
use crate::discover::{AcpiTouchpad, Found};
use crate::driver::Driver;
use crate::transaction::probe;

/// Where HID-over-I2C touchpads commonly ship: ELAN, Synaptics, then ALPS.
const SHIPPED_AT: [u8; 3] = [0x15, 0x2C, 0x10];

/// The first SHIPPED_AT address to acknowledge a one-byte read on
/// `driver`'s bus, with the HID descriptor register of the declaration with
/// no address that names `dev`; None, with nothing put on the bus, when no
/// such declaration does.
pub fn sweep(driver: &Driver, dev: &Found, targets: &[AcpiTouchpad]) -> Option<(u8, u16)> {
    let unaddressed = |t: &&AcpiTouchpad| t.addr == 0 && t.info & HID_INFO_TEN_BIT == 0;
    let reg = targets.iter().filter(unaddressed).find(|t| t.names(dev))?.desc_reg;
    // A bus that faults answers nothing after: the sweep ends there too.
    let mut answers =
        SHIPPED_AT.into_iter().map_while(|a| probe(driver, a).ok().map(|ack| (a, ack)));
    answers.find(|&(_, ack)| ack).map(|(a, _)| (a, reg))
}
