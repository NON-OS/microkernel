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

/// An endpoint held to a list, then the endpoints whose owners may send to it.
pub(super) const HELD: &[(&str, &[&str])] = &[
    ("driver.virtio_net0", WIRED_STACK),
    ("driver.e1000_0", WIRED_STACK),
    ("driver.rtl8169_0", WIRED_STACK),
    ("driver.rtl8139_0", WIRED_STACK),
    ("driver.iwlwifi0", WIFI_STACK),
    ("driver.rtl8821ce0", WIFI_STACK),
    ("driver.ps2_kbd0", KERNEL_ONLY),
    ("driver.usb_hid0", KERNEL_ONLY),
    ("driver.i2c_hid0", KERNEL_ONLY),
    ("driver.usb_msc0", KERNEL_ONLY),
    ("driver.virtio_rng", KERNEL_ONLY),
    ("driver.xhci0", &["driver.usb_hid0", "driver.usb_msc0"]),
    ("driver.i2c_pci0", &["driver.i2c_hid0"]),
    ("driver.virtio_gpu0", &["compositor"]),
    ("driver.hda0", &["audio.server"]),
];

/// The kernel polls the keyboards and pointers, reads the random source and
/// serves USB storage itself; no capsule has reason to reach them.
const KERNEL_ONLY: &[&str] = &[];

/// net.core carries the frames and asks each card for its link; net.l2 finds
/// the wired cards.
const WIRED_STACK: &[&str] = &["net.core", "net.l2"];
/// A Wi-Fi card answers net.core, which carries its frames, and the panels
/// that scan and join: Settings, in its first window or a later one
/// (`src/userspace/capsule_settings/spawn.rs`), and the setup wizard. net.l2
/// binds only a wired card.
const WIFI_STACK: &[&str] =
    &["net.core", "app.settings", "app.settings.1", "app.settings.2", "app.setup_wizard"];
