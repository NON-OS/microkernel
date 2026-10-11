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

//! The network cards a hypervisor emulates and no physical machine carries,
//! each spawned only where the device table has its device.

pub(super) fn spawn() {
    spawn_pcnet();
    spawn_vmxnet3();
}

#[cfg(feature = "nonos-capsule-driver-pcnet")]
fn spawn_pcnet() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::pcnet_capsule as c;
    if !super::device_present::present("DRIVER-PCNET", HardwareFamily::NetworkPcnet) {
        return;
    }
    super::boot::capsule(
        "DRIVER-PCNET",
        "driver_pcnet",
        c::spawn_driver_pcnet_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-pcnet"))]
fn spawn_pcnet() {}

#[cfg(feature = "nonos-capsule-driver-vmxnet3")]
fn spawn_vmxnet3() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::vmxnet3_capsule as c;
    if !super::device_present::present("DRIVER-VMXNET3", HardwareFamily::NetworkVmxnet3) {
        return;
    }
    super::boot::capsule(
        "DRIVER-VMXNET3",
        "driver_vmxnet3",
        c::spawn_driver_vmxnet3_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-vmxnet3"))]
fn spawn_vmxnet3() {}
