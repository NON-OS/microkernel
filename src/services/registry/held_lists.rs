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
/// serves USB storage itself; no capsule has reason to reach them. The SMBus
/// host has no driver above it yet: a raw transfer reaches any device on the
/// bus, so it is opened to one when one exists.
pub(super) const KERNEL_ONLY: &[&str] = &[];

/// The class drivers above the xHCI controller, each searching the ports
/// for its own kind of device.
pub(super) const XHCI_CLIENTS: &[&str] = &[
    "driver.usb_hid0",
    "driver.usb_msc0",
    "driver.usb_audio0",
    "driver.cdc_ecm0",
    "driver.cdc_ncm0",
    "driver.rndis0",
    "driver.ax88179_0",
    "driver.rtl8153_0",
];

/// net.core carries the frames and asks each card for its link; net.l2 finds
/// the wired cards.
pub(super) const WIRED_STACK: &[&str] = &["net.core", "net.l2"];
/// A Wi-Fi card answers net.core, which carries its frames, and the panels
/// that scan and join: Settings, in its first window or a later one
/// (`src/userspace/capsule_settings/spawn.rs`), and the setup wizard. net.l2
/// binds only a wired card.
pub(super) const WIFI_STACK: &[&str] =
    &["net.core", "app.settings", "app.settings.1", "app.settings.2", "app.setup_wizard"];

/// The Bluetooth host answers Settings, which scans, pairs and forgets in any
/// of its windows, and the shell, which shows the link and battery levels;
/// the capsule checks each op again (letter E-25).
pub(super) const BLUETOOTH_PANELS: &[&str] =
    &["app.settings", "app.settings.1", "app.settings.2", "desktop_shell"];
