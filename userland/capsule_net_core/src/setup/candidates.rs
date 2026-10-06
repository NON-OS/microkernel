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

// WiFi links are checked first. On a laptop the user explicitly associates, so an
// up WiFi link is the one they want; and a cable-less wired NIC can still claim
// carrier, which would strand the stack on a dead port. Wired links (a QEMU
// virtio-net or a real cabled NIC) are the fallback and still win when no WiFi
// link is up, so the desktop/QEMU path is unchanged.
const WIFI_NICS: &[&str] = &["driver.iwlwifi0", "driver.rtl8821ce0"];
// The USB network drivers come after the PCI ones: a USB Ethernet adapter
// or a tethering phone is bound when no card has its link up, and a phone
// plugged in later is found on the next pass.
const WIRED_NICS: &[&str] = &[
    "driver.virtio_net0",
    "driver.e1000_0",
    "driver.rtl8169_0",
    "driver.rtl8139_0",
    "driver.cdc_ecm0",
    "driver.cdc_ncm0",
    "driver.rndis0",
    "driver.ax88179_0",
    "driver.rtl8153_0",
];

/// How many candidates there are, WiFi and wired.
pub(super) fn count() -> usize {
    WIFI_NICS.len() + WIRED_NICS.len()
}

/// The NIC name at `idx`, counting WiFi candidates before wired ones.
pub(super) fn candidate(idx: usize) -> &'static str {
    if idx < WIFI_NICS.len() {
        WIFI_NICS[idx]
    } else {
        WIRED_NICS[idx - WIFI_NICS.len()]
    }
}
