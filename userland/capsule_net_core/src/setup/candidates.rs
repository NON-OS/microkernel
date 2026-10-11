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

//! The NIC drivers the stack may bind, by service name.

// The kernel's table of the cards it starts and holds to this stack.
#[path = "../../../../src/services/registry/nics.rs"]
mod nics;

use nics::{Link, NICS};

// WiFi links are checked first. On a laptop the user explicitly associates, so an
// up WiFi link is the one they want; and a cable-less wired NIC can still claim
// carrier, which would strand the stack on a dead port. Wired links (a QEMU
// virtio-net or a real cabled NIC) are the fallback and still win when no WiFi
// link is up, so the desktop/QEMU path is unchanged.
//
// The USB network drivers come after the PCI ones: a USB Ethernet adapter
// or a tethering phone is bound when no card has its link up, and a phone
// plugged in later is found on the next pass. The kernel starts none of them,
// so they are not on its list.
const USB_NICS: &[&str] = &[
    "driver.cdc_ecm0",
    "driver.cdc_ncm0",
    "driver.rndis0",
    "driver.ax88179_0",
    "driver.rtl8153_0",
];

/// How many candidates there are, WiFi, wired and USB.
pub(super) fn count() -> usize {
    NICS.len() + USB_NICS.len()
}

/// The NIC name at `idx`, counting WiFi candidates, then wired, then USB.
/// discover_nic asks only below `count()`; past it, an empty name finds no
/// service.
pub(super) fn candidate(idx: usize) -> &'static str {
    let of = |link| NICS.iter().filter(move |(_, l)| *l == link).map(|(card, _)| *card);
    let mut all = of(Link::Wifi).chain(of(Link::Wired)).chain(USB_NICS.iter().copied());
    all.nth(idx).unwrap_or("")
}
