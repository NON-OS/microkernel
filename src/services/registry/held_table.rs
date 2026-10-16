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

//! Which endpoints are held, and to whom (held.rs says why).

use super::super::usb_hosts::USB_HOSTS;

/// An endpoint held to a list, then the endpoints whose owners may send to it.
pub(super) const HELD: &[(&str, &[&str])] = &[
    ("driver.ps2_kbd0", KERNEL_ONLY),
    ("driver.usb_hid0", KERNEL_ONLY),
    ("driver.i2c_hid0", KERNEL_ONLY),
    ("driver.usb_msc0", KERNEL_ONLY),
    ("driver.usb_hub0", KERNEL_ONLY),
    ("driver.smbus0", KERNEL_ONLY),
    ("driver.virtio_rng", KERNEL_ONLY),
    ("driver.ccid0", KERNEL_ONLY),
    ("driver.fido0", KERNEL_ONLY),
    ("driver.spi0", &["driver.spi_hid0"]),
    ("driver.spi_hid0", KERNEL_ONLY),
    (USB_HOSTS[0], USB_CLIENTS),
    ("driver.i2c_pci0", &["driver.i2c_hid0"]),
    ("driver.virtio_gpu0", &["compositor"]),
    ("driver.bga0", &["compositor"]),
    ("driver.hda0", &["audio.server"]),
    ("driver.usb_audio0", &["audio.server"]),
    // A modem dials out, outside the net stack: whom else is the owner's.
    ("driver.cdc_acm0", &["app.terminal"]),
    (USB_HOSTS[1], USB_CLIENTS),
    (USB_HOSTS[2], USB_CLIENTS),
    (USB_HOSTS[3], USB_CLIENTS),
];

/// The kernel polls the keyboards and pointers, reads the random source and
/// serves USB storage itself, and the hub walker answers it alone; no
/// capsule has reason to reach them. The SMBus
/// host has no driver above it yet: a raw transfer reaches any device on the
/// bus, so it is opened to one when one exists. A smart card signs and
/// decrypts for whoever reaches its reader, and a security key signs for
/// whoever reaches it once its user touches it: the owner names who.
/// The SPI host moves raw bytes to any device on its bus: only the HID
/// driver above it reaches it, and that driver posts input, serving no one.
const KERNEL_ONLY: &[&str] = &[];

/// The hub walker and the class drivers above every USB host (usb_hosts.rs),
/// each searching the ports, root and virtual, for its own kind of device.
const USB_CLIENTS: &[&str] = &[
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
    "driver.ax88179_0",
    "driver.rtl8153_0",
];
