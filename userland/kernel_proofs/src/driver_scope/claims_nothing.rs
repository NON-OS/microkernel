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

//! The drivers the kernel spawns that hold no scope row (spawned.rs).

/// Drivers that reach their devices through another driver, or vmmouse0
/// through the backdoor alone, and never call MkDeviceClaim: they have no
/// row, and must not gain one unnoticed.
pub(super) const CLAIMS_NOTHING: [&str; 15] = [
    "driver.usb_hid0",
    "driver.usb_msc0",
    "driver.i2c_hid0",
    "driver.vmmouse0",
    "driver.ccid0",
    "driver.cdc_acm0",
    "driver.cdc_ecm0",
    "driver.cdc_ncm0",
    "driver.fido0",
    "driver.rndis0",
    "driver.spi_hid0",
    "driver.usb_audio0",
    "driver.usb_hub0",
    "driver.usb_mic0",
    "driver.uvc0",
];
