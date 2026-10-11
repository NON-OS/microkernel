// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The SD host controller capsule, started for the eMMC hosts the
//! inventory knows, which it took over from the AHCI capsule, and for any
//! other SD host: a card reader is served on a machine with no eMMC.

#[cfg(feature = "nonos-capsule-driver-sdhci")]
pub(super) fn spawn() {
    use crate::hardware::inventory::{present as has, HardwareFamily};
    use crate::hardware::sdhci_capsule as c;
    let present = super::device_present::present;
    if !has(HardwareFamily::StorageEmmc) && !present("DRIVER-SDHCI", HardwareFamily::StorageSdhci) {
        return;
    }
    super::boot::capsule(
        "DRIVER-SDHCI",
        "driver_sdhci",
        c::spawn_driver_sdhci_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-sdhci"))]
pub(super) fn spawn() {}
