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
// along with this program. If not, see <https://www.gnu.org/licenses/>.//! The Intel PCIe cards that are not in every virtual machine: each starts
//! only when the inventory found its hardware (`device_present`), so one that
//! finds none never runs.

pub(super) fn spawn() {
    spawn_e1000e();
    spawn_igb();
    spawn_ixgbe();
}

#[cfg(feature = "nonos-capsule-driver-e1000e")]
fn spawn_e1000e() {
    use crate::hardware::e1000e_capsule as c;
    use crate::hardware::inventory::HardwareFamily;
    if !super::super::device_present::present("DRIVER-E1000E", HardwareFamily::NetworkE1000e) {
        return;
    }
    let spawn = c::spawn_driver_e1000e_capsule;
    super::super::boot::capsule("DRIVER-E1000E", "driver_e1000e", spawn, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-e1000e"))]
fn spawn_e1000e() {}

#[cfg(feature = "nonos-capsule-driver-igb")]
fn spawn_igb() {
    use crate::hardware::igb_capsule as c;
    use crate::hardware::inventory::HardwareFamily;
    if !super::super::device_present::present("DRIVER-IGB", HardwareFamily::NetworkIgb) {
        return;
    }
    super::super::boot::capsule(
        "DRIVER-IGB",
        "driver_igb",
        c::spawn_driver_igb_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-igb"))]
fn spawn_igb() {}

#[cfg(feature = "nonos-capsule-driver-ixgbe")]
fn spawn_ixgbe() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::ixgbe_capsule as c;
    if !super::super::device_present::present("DRIVER-IXGBE", HardwareFamily::NetworkIxgbe) {
        return;
    }
    let spawn = c::spawn_driver_ixgbe_capsule;
    super::super::boot::capsule("DRIVER-IXGBE", "driver_ixgbe", spawn, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-ixgbe"))]
fn spawn_ixgbe() {}
