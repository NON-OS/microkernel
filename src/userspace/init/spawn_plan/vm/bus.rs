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

//! The buses a hypervisor offers devices on, before the devices: VMBus,
//! spawned where the Hyper-V SynIC is up and its root recorded.

pub(in crate::userspace::init::spawn_plan) fn spawn() {
    spawn_vmbus();
}

#[cfg(feature = "nonos-capsule-driver-vmbus")]
fn spawn_vmbus() {
    use crate::hardware::vmbus_capsule as c;
    if !crate::arch::hv_vmbus_ready() {
        crate::sys::boot_log::ok("DRIVER-VMBUS", "no Hyper-V SynIC, not spawned");
        return;
    }
    super::super::boot::capsule(
        "DRIVER-VMBUS",
        "driver_vmbus",
        c::spawn_driver_vmbus_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-vmbus"))]
fn spawn_vmbus() {}
