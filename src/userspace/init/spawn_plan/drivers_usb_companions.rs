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

//! The USB 1.1 hosts' drivers, after EHCI's: on a chipset with EHCI they
//! are its companions and serve the full- and low-speed devices it hands
//! on; alone (a VM's OHCI or UHCI), they serve every device.

pub(super) fn spawn() {
    spawn_ohci();
    spawn_uhci();
}

/// An OHCI controller: VirtualBox's, QEMU's, AMD, SiS, NEC and Apple chipsets'.
#[cfg(feature = "nonos-capsule-driver-ohci")]
fn spawn_ohci() {
    use crate::hardware::inventory::{present, HardwareFamily};
    use crate::hardware::ohci_capsule as c;
    if !present(HardwareFamily::UsbOhci) {
        crate::sys::boot_log::ok("DRIVER-OHCI", "no controller present, not spawned");
        return;
    }
    let spawn = c::spawn_driver_ohci_capsule;
    super::boot::capsule("DRIVER-OHCI", "driver_ohci", spawn, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-driver-ohci"))]
fn spawn_ohci() {}

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
