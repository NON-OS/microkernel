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

use super::super::nics::{Link, NICS};

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
    ("driver.ps2_kbd0", KERNEL_ONLY),
    ("driver.usb_hid0", KERNEL_ONLY),
    ("driver.i2c_hid0", KERNEL_ONLY),
    ("driver.usb_msc0", KERNEL_ONLY),
    ("driver.smbus0", KERNEL_ONLY),
    ("driver.virtio_rng", KERNEL_ONLY),
    ("driver.xhci0", &["driver.usb_hid0", "driver.usb_msc0", "driver.usb_audio0"]),
    ("driver.i2c_pci0", &["driver.i2c_hid0"]),
    ("driver.virtio_gpu0", &["compositor"]),
    ("driver.bga0", &["compositor"]),
    ("driver.hda0", &["audio.server"]),
    ("driver.usb_audio0", &["audio.server"]),
];

/// The kernel polls the keyboards and pointers, reads the random source and
/// serves USB storage itself; no capsule has reason to reach them. The SMBus
/// host has no driver above it yet: a raw transfer reaches any device on the
/// bus, so it is opened to one when one exists.
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
