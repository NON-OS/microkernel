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

// The lists the entries name sit beside this file, mounted by path for the
// same reason held.rs mounts this one.
#[path = "held_lists.rs"]
mod lists;

use super::super::nics::{Link, NICS};
use lists::{BLUETOOTH_PANELS, KERNEL_ONLY, WIFI_STACK, WIRED_STACK, XHCI_CLIENTS};

/// The stack a network card is held to, when `endpoint` is one. The cards
/// come from nics.rs, so a card the stack binds is never left unheld.
pub(super) fn card_callers(endpoint: &str) -> Option<&'static [&'static str]> {
    let (_, link) = NICS.iter().find(|(card, _)| *card == endpoint)?;
    Some(match link {
        Link::Wired => WIRED_STACK,
        Link::Wifi => WIFI_STACK,
    })
}

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
    ("driver.smbus0", KERNEL_ONLY),
    ("driver.virtio_rng", KERNEL_ONLY),
    ("driver.xhci0", XHCI_CLIENTS),
    ("driver.i2c_pci0", &["driver.i2c_hid0"]),
    ("driver.virtio_gpu0", &["compositor"]),
    ("driver.bga0", &["compositor"]),
    ("driver.vmware_svga0", &["compositor"]),
    ("driver.hda0", &["audio.server"]),
    ("driver.usb_audio0", &["audio.server"]),
    ("driver.ac97_0", &["audio.server"]),
    // The GPIO controller's pins are held by the bus controller whose
    // device's GpioInt names them; more holders join as their drivers do.
    ("driver.gpio0", &["driver.i2c_pci0"]),
    ("acpi", &["power", "desktop_shell", "driver.i2c_pci0", "driver.i2c_hid0", "driver.gpio0"]),
    // The wiped power-off: only the shell asks for it (F-88).
    ("power.off", &["desktop_shell"]),
];
