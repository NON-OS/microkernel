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

//! Which endpoints are held, and to whom (held.rs says why). The display
//! drivers are held to the compositor from displays.rs and the network cards
//! to their stacks from nics.rs (held_cards.rs), not listed here.

// The lists the entries name sit beside this file, mounted by path for the
// same reason held.rs mounts this one.
#[path = "held_lists.rs"]
mod lists;

use super::super::usb_hosts::USB_HOSTS;
use lists::{BLUETOOTH_PANELS, KERNEL_ONLY, USB_CLIENTS};

/// An endpoint held to a list, then the endpoints whose owners may send to it.
pub(super) const HELD: &[(&str, &[&str])] = &[
    ("driver.bt_usb0", BLUETOOTH_PANELS),
    ("driver.ps2_kbd0", KERNEL_ONLY),
    ("driver.usb_hid0", KERNEL_ONLY),
    ("driver.i2c_hid0", KERNEL_ONLY),
    ("driver.virtio_input0", KERNEL_ONLY),
    ("driver.vmmdev0", KERNEL_ONLY),
    ("driver.vmmouse0", KERNEL_ONLY),
    ("driver.vmbus0", KERNEL_ONLY),
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
    ("driver.hda0", &["audio.server"]),
    ("driver.ac97_0", &["audio.server"]),
    ("driver.usb_audio0", &["audio.server"]),
    // A modem dials out, outside the net stack: whom else is the owner's.
    ("driver.cdc_acm0", &["app.terminal"]),
    (USB_HOSTS[1], USB_CLIENTS),
    (USB_HOSTS[2], USB_CLIENTS),
    (USB_HOSTS[3], USB_CLIENTS),
    // The GPIO controller's pins are held by the bus controllers whose
    // devices' GpioInt names them; more holders join as their drivers do.
    ("driver.gpio0", &["driver.i2c_pci0", "driver.spi0"]),
    ("acpi", &["power", "desktop_shell", "driver.i2c_pci0", "driver.i2c_hid0", "driver.gpio0"]),
    // The wiped power-off: only the shell asks for it (F-88). `power` shares
    // capsule_power's inbox, so this line gates both: the installers are on
    // it for their restart (S-14), and the capsule, by the sender's pid,
    // keeps the power-off and shutdown the shell's (off/who.rs, rule.rs).
    ("power.off", &["desktop_shell", "app.install", "tool.install"]),
];

/// A display driver shows whatever it is given over the whole screen: only
/// the compositor drives one (every name in displays.rs).
pub(super) const DISPLAY_CALLERS: &[&str] = &["compositor"];
