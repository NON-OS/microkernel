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

//! The input devices a hypervisor emulates, spawned only where the device
//! table has one, or for the backdoor's pointer, where the backdoor answers.

pub(in crate::userspace::init::spawn_plan) fn spawn() {
    spawn_virtio_input();
    spawn_vmmdev();
    spawn_vmmouse();
}

#[cfg(feature = "nonos-capsule-driver-virtio-input")]
fn spawn_virtio_input() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::virtio_input_capsule as c;
    if !super::present::present("DRIVER-VIRTIO-INPUT", HardwareFamily::InputVirtio) {
        return;
    }
    super::super::boot::capsule(
        "DRIVER-VIRTIO-INPUT",
        "driver_virtio_input",
        c::spawn_driver_virtio_input_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-virtio-input"))]
fn spawn_virtio_input() {}

#[cfg(feature = "nonos-capsule-driver-vmmdev")]
fn spawn_vmmdev() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::vmmdev_capsule as c;
    if !super::present::present("DRIVER-VMMDEV", HardwareFamily::InputVmmdev) {
        return;
    }
    super::super::boot::capsule(
        "DRIVER-VMMDEV",
        "driver_vmmdev",
        c::spawn_driver_vmmdev_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-vmmdev"))]
fn spawn_vmmdev() {}

/*
 * The backdoor's pointer is no PCI device: it answers on the hypervisor's
 * port, which VMware always has and QEMU has with vmport. The kernel asks
 * GETVERSION once here, so a host without it costs no spawn.
 */
#[cfg(feature = "nonos-capsule-driver-vmmouse")]
fn spawn_vmmouse() {
    use crate::hardware::vmmouse_capsule as c;
    if !crate::arch::hv_backdoor_present() {
        crate::sys::boot_log::ok("DRIVER-VMMOUSE", "no backdoor on this host, not spawned");
        return;
    }
    super::super::boot::capsule(
        "DRIVER-VMMOUSE",
        "driver_vmmouse",
        c::spawn_driver_vmmouse_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-vmmouse"))]
fn spawn_vmmouse() {}
