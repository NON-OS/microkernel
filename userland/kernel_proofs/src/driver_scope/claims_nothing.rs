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
//! driver.bt_usb0 claims no PCI device: its controller is a USB device
//! behind driver.xhci0, the RTL8821CE's Bluetooth half among them.

/// Drivers that reach their devices through another driver, or vmmouse0
/// through the backdoor alone, and never call MkDeviceClaim: they have no
/// row, and must not gain one unnoticed.
pub(super) const CLAIMS_NOTHING: [&str; 18] = [
    "driver.usb_hid0",
    "driver.bt_usb0",
    "driver.usb_msc0",
    "driver.i2c_hid0",
    "driver.vmmouse0",
    "driver.ccid0",
    "driver.cdc_acm0",
    "driver.cdc_ecm0",
    "driver.cdc_ncm0",
    "driver.fido0",
    "driver.rndis0",
    "driver.ax88179_0",
    "driver.rtl8153_0",
    "driver.spi_hid0",
    "driver.usb_audio0",
    "driver.usb_hub0",
    "driver.usb_mic0",
    "driver.uvc0",
];
