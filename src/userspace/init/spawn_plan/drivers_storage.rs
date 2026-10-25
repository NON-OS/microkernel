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

#[cfg(any(feature = "nonos-capsule-driver-ahci", feature = "nonos-capsule-driver-nvme"))]
use crate::hardware::inventory::HardwareFamily;

/// The SATA and NVMe controllers, before the bus drivers (orchestrator.rs):
/// a disk controller that DMAs claims its isolation group ahead of anything
/// sharing it.
pub(super) fn spawn_disks() {
    spawn_ahci();
    spawn_nvme();
}

pub(super) fn spawn() {
    spawn_hda();
    super::drivers_sdhci::spawn();
    super::drivers_ufs::spawn();
}

#[cfg(feature = "nonos-capsule-driver-ahci")]
fn spawn_ahci() {
    use crate::hardware::ahci_capsule as c;
    if !super::device_present::present("DRIVER-AHCI", HardwareFamily::StorageAhci) {
        return;
    }
    super::boot::capsule(
        "DRIVER-AHCI",
        "driver_ahci",
        c::spawn_driver_ahci_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-ahci"))]
fn spawn_ahci() {}

#[cfg(feature = "nonos-capsule-driver-hda")]
fn spawn_hda() {
    use crate::hardware::hda_capsule as c;
    super::boot::capsule("DRIVER-HDA", "driver_hda", c::spawn_driver_hda_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-driver-hda"))]
fn spawn_hda() {}

#[cfg(feature = "nonos-capsule-driver-nvme")]
fn spawn_nvme() {
    use crate::hardware::nvme_capsule as c;
    if !super::device_present::present("DRIVER-NVME", HardwareFamily::StorageNvme) {
        return;
    }
    super::boot::capsule(
        "DRIVER-NVME",
        "driver_nvme",
        c::spawn_driver_nvme_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-nvme"))]
fn spawn_nvme() {}
