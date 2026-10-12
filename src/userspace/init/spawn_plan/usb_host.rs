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
//! Whether the machine has a USB host this image's drivers serve. The USB
//! class drivers start only then, and each searches every host
//! (services/registry/usb_hosts.rs) for its device.

// Each USB class driver's spawn asks it first.
#![cfg(any(
    feature = "nonos-capsule-driver-usb-msc",
    feature = "nonos-capsule-driver-usb-audio",
    feature = "nonos-capsule-driver-cdc-ecm",
    feature = "nonos-capsule-driver-cdc-ncm",
    feature = "nonos-capsule-driver-rndis",
    feature = "nonos-capsule-driver-ax88179",
    feature = "nonos-capsule-driver-rtl8153"
))]

use crate::hardware::inventory::{present, HardwareFamily};

/// The host families a driver exists for; OHCI and EHCI join with theirs.
const DRIVEN: [HardwareFamily; 2] = [HardwareFamily::UsbXhci, HardwareFamily::UsbUhci];

pub(super) fn usb_host() -> bool {
    DRIVEN.iter().any(|&f| present(f))
}
