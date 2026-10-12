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

//! The USB host controllers' drivers, each where the inventory saw its
//! controller. The class drivers find them by usb_hosts.rs.

pub(super) fn spawn() {
    spawn_xhci();
    spawn_uhci();
}

#[cfg(feature = "nonos-capsule-driver-xhci")]
fn spawn_xhci() {
    use crate::hardware::xhci_capsule as c;
    super::boot::capsule(
        "DRIVER-XHCI",
        "driver_xhci",
        c::spawn_driver_xhci_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-xhci"))]
fn spawn_xhci() {}

/// A UHCI controller: QEMU's PIIX3, VMware's, Intel and VIA chipsets.
#[cfg(feature = "nonos-capsule-driver-uhci")]
fn spawn_uhci() {
    use crate::hardware::inventory::{present, HardwareFamily};
    use crate::hardware::uhci_capsule as c;
    if !present(HardwareFamily::UsbUhci) {
        crate::sys::boot_log::ok("DRIVER-UHCI", "no controller present, not spawned");
        return;
    }
    let spawn = c::spawn_driver_uhci_capsule;
    super::boot::capsule("DRIVER-UHCI", "driver_uhci", spawn, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-driver-uhci"))]
fn spawn_uhci() {}
