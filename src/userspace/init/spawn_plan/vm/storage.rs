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

//! The disk controllers hypervisors emulate and no current machine carries,
//! each spawned only where the device table has its controller, before vfs,
//! which looks for the store's disk.

pub(in crate::userspace::init::spawn_plan) fn spawn() {
    spawn_piix_ide();
    spawn_virtio_scsi();
    spawn_pvscsi();
    spawn_buslogic();
    spawn_mpt();
}

#[cfg(feature = "nonos-capsule-driver-piix-ide")]
fn spawn_piix_ide() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::piix_ide_capsule as c;
    if !super::present::present("DRIVER-PIIX-IDE", HardwareFamily::StorageIde) {
        return;
    }
    super::super::boot::capsule(
        "DRIVER-PIIX-IDE",
        "driver_piix_ide",
        c::spawn_driver_piix_ide_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-piix-ide"))]
fn spawn_piix_ide() {}

#[cfg(feature = "nonos-capsule-driver-virtio-scsi")]
fn spawn_virtio_scsi() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::virtio_scsi_capsule as c;
    if !super::present::present("DRIVER-VIRTIO-SCSI", HardwareFamily::StorageVirtioScsi) {
        return;
    }
    super::super::boot::capsule(
        "DRIVER-VIRTIO-SCSI",
        "driver_virtio_scsi",
        c::spawn_driver_virtio_scsi_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-virtio-scsi"))]
fn spawn_virtio_scsi() {}

#[cfg(feature = "nonos-capsule-driver-pvscsi")]
fn spawn_pvscsi() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::pvscsi_capsule as c;
    if !super::present::present("DRIVER-PVSCSI", HardwareFamily::StorageVmwarePvscsi) {
        return;
    }
    super::super::boot::capsule(
        "DRIVER-PVSCSI",
        "driver_pvscsi",
        c::spawn_driver_pvscsi_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-pvscsi"))]
fn spawn_pvscsi() {}

#[cfg(feature = "nonos-capsule-driver-buslogic")]
fn spawn_buslogic() {
    use crate::hardware::buslogic_capsule as c;
    use crate::hardware::inventory::HardwareFamily;
    if !super::present::present("DRIVER-BUSLOGIC", HardwareFamily::StorageBuslogic) {
        return;
    }
    super::super::boot::capsule(
        "DRIVER-BUSLOGIC",
        "driver_buslogic",
        c::spawn_driver_buslogic_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-buslogic"))]
fn spawn_buslogic() {}

#[cfg(feature = "nonos-capsule-driver-mpt")]
fn spawn_mpt() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::mpt_capsule as c;
    if !super::present::present("DRIVER-MPT", HardwareFamily::StorageMpt) {
        return;
    }
    super::super::boot::capsule(
        "DRIVER-MPT",
        "driver_mpt",
        c::spawn_driver_mpt_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-mpt"))]
fn spawn_mpt() {}
