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

//! The display adapters a hypervisor emulates, each spawned only where the
//! device table has its device.

pub(in crate::userspace::init::spawn_plan) fn spawn() {
    spawn_vmware_svga();
}

#[cfg(feature = "nonos-capsule-driver-vmware-svga")]
fn spawn_vmware_svga() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::vmware_svga_capsule as c;
    if !super::present::present("DRIVER-VMWARE-SVGA", HardwareFamily::DisplayVmwareSvga) {
        return;
    }
    super::super::boot::capsule(
        "DRIVER-VMWARE-SVGA",
        "driver_vmware_svga",
        c::spawn_driver_vmware_svga_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-vmware-svga"))]
fn spawn_vmware_svga() {}
