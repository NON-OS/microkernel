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

//! The UFS host's driver, started only on a machine with a UFS host.

#[cfg(feature = "nonos-capsule-driver-ufs")]
pub(super) fn spawn() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::ufs_capsule as c;
    if !super::device_present::present("DRIVER-UFS", HardwareFamily::StorageUfs) {
        return;
    }
    super::boot::capsule("DRIVER-UFS", "driver_ufs", c::spawn_driver_ufs_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-driver-ufs"))]
pub(super) fn spawn() {}
