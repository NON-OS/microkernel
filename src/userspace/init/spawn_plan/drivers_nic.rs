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

mod realtek;

pub(super) fn spawn() {
    spawn_e1000();
    spawn_e1000e();
    spawn_igb();
    spawn_ixgbe();
    spawn_tg3();
    realtek::spawn();
}

#[cfg(feature = "nonos-capsule-driver-e1000")]
fn spawn_e1000() {
    use crate::hardware::e1000_capsule as c;
    super::boot::capsule(
        "DRIVER-E1000",
        "driver_e1000",
        c::spawn_driver_e1000_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-e1000"))]
fn spawn_e1000() {}

#[cfg(feature = "nonos-capsule-driver-e1000e")]
fn spawn_e1000e() {
    use crate::hardware::e1000e_capsule as c;
    let spawn = c::spawn_driver_e1000e_capsule;
    super::boot::capsule("DRIVER-E1000E", "driver_e1000e", spawn, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-e1000e"))]
fn spawn_e1000e() {}

#[cfg(feature = "nonos-capsule-driver-igb")]
fn spawn_igb() {
    use crate::hardware::igb_capsule as c;
    super::boot::capsule("DRIVER-IGB", "driver_igb", c::spawn_driver_igb_capsule, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-igb"))]
fn spawn_igb() {}

#[cfg(feature = "nonos-capsule-driver-ixgbe")]
fn spawn_ixgbe() {
    use crate::hardware::ixgbe_capsule as c;
    let spawn = c::spawn_driver_ixgbe_capsule;
    super::boot::capsule("DRIVER-IXGBE", "driver_ixgbe", spawn, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-ixgbe"))]
fn spawn_ixgbe() {}

#[cfg(feature = "nonos-capsule-driver-tg3")]
fn spawn_tg3() {
    use crate::hardware::tg3_capsule as c;
    let spawn = c::spawn_driver_tg3_capsule;
    super::boot::capsule("DRIVER-TG3", "driver_tg3", spawn, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-tg3"))]
fn spawn_tg3() {}
