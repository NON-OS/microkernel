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
//! inventory knows. It took them over from the AHCI capsule, which no
//! longer starts for an eMMC host.

#[cfg(feature = "nonos-capsule-driver-sdhci")]
pub(super) fn spawn() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::sdhci_capsule as c;
    if !super::device_present::present("DRIVER-SDHCI", HardwareFamily::StorageEmmc) {
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
