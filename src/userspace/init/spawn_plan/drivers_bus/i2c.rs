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

//! The I2C host capsule and the HID capsule that asks it for its touchpad,
//! started only where the broker lists a host the first may drive
//! (inventory::i2c_host). The host capsule finds the touchpad from the
//! firmware's declaration or its own read-only sweep, so the HID capsule
//! starts with it, and without it has nothing to ask.

#[cfg(any(feature = "nonos-capsule-driver-i2c-pci", feature = "nonos-capsule-driver-i2c-hid"))]
pub(super) fn spawn() {
    if !crate::hardware::inventory::i2c_host_present() {
        crate::sys::boot_log::ok("DRIVER-I2C-PCI", "no controller present, not spawned");
        crate::sys::boot_log::ok("DRIVER-I2C-HID", "no I2C host to ask, not spawned");
        return;
    }
    spawn_i2c_pci();
    spawn_i2c_hid();
}

#[cfg(not(any(
    feature = "nonos-capsule-driver-i2c-pci",
    feature = "nonos-capsule-driver-i2c-hid"
)))]
pub(super) fn spawn() {}

#[cfg(feature = "nonos-capsule-driver-i2c-pci")]
fn spawn_i2c_pci() {
    use crate::hardware::i2c_pci_capsule as c;
    super::super::boot::capsule(
        "DRIVER-I2C-PCI",
        "driver_i2c_pci",
        c::spawn_driver_i2c_pci_capsule,
        c::shared_state,
    );
}

#[cfg(all(
    not(feature = "nonos-capsule-driver-i2c-pci"),
    feature = "nonos-capsule-driver-i2c-hid"
))]
fn spawn_i2c_pci() {}

#[cfg(feature = "nonos-capsule-driver-i2c-hid")]
fn spawn_i2c_hid() {
    use crate::userspace::capsule_driver_i2c_hid as c;
    super::super::boot::capsule(
        "DRIVER-I2C-HID",
        "driver_i2c_hid",
        c::spawn_driver_i2c_hid_capsule,
        c::shared_state,
    );
}

#[cfg(all(
    not(feature = "nonos-capsule-driver-i2c-hid"),
    feature = "nonos-capsule-driver-i2c-pci"
))]
fn spawn_i2c_hid() {}
