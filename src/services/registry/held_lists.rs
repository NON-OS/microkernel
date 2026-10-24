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

//! The callers the held endpoints are held to (held_table.rs).

/// The kernel polls the keyboards and pointers, reads the random source and
/// serves USB storage itself, and the hub walker answers it alone; no capsule
/// has reason to reach them. The SMBus host has no driver above it yet: a raw
/// transfer reaches any device on the bus, so it is opened to one when one
/// exists. A smart card signs and decrypts for whoever reaches its reader,
/// and a security key signs for whoever reaches it once its user touches it:
/// the owner names who. The SPI host moves raw bytes to any device on its
/// bus: only the HID driver above it reaches it, and that driver posts input,
/// serving no one.
pub(super) const KERNEL_ONLY: &[&str] = &[];

/// The hub walker and class drivers above every USB host (usb_hosts.rs),
/// each finding its own kind of device.
pub(super) const USB_CLIENTS: &[&str] = &[
    "driver.usb_hub0",
    "driver.usb_hid0",
    "driver.usb_msc0",
    "driver.usb_audio0",
    "driver.uvc0",
    "driver.cdc_acm0",
    "driver.usb_mic0",
    "driver.ccid0",
    "driver.fido0",
    "driver.cdc_ecm0",
    "driver.cdc_ncm0",
    "driver.rndis0",
];

/// The Bluetooth host answers Settings, which scans, pairs and forgets in any
/// of its windows, and the shell, which shows the link and battery levels;
/// the capsule checks each op again (letter E-25).
pub(super) const BLUETOOTH_PANELS: &[&str] =
    &["app.settings", "app.settings.1", "app.settings.2", "desktop_shell"];
