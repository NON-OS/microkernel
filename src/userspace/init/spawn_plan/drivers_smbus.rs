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

//! The Intel SMBus host, only where the inventory saw one.

pub(super) fn spawn() {
    spawn_smbus();
}

#[cfg(feature = "nonos-capsule-driver-smbus")]
fn spawn_smbus() {
    use crate::hardware::inventory::{present, HardwareFamily};
    use crate::hardware::smbus_capsule as c;
    if !present(HardwareFamily::SerialSmbus) {
        crate::sys::boot_log::ok("DRIVER-SMBUS", "no controller present, not spawned");
        return;
    }
    super::boot::capsule(
        "DRIVER-SMBUS",
        "driver_smbus",
        c::spawn_driver_smbus_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-smbus"))]
fn spawn_smbus() {}
