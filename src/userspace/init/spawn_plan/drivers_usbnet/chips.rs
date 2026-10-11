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

//! The USB Ethernet chips with drivers of their own, after the class
//! drivers; the parent has already found an xHCI controller.

pub(super) fn spawn() {
    spawn_ax88179();
    spawn_rtl8153();
}

#[cfg(feature = "nonos-capsule-driver-ax88179")]
fn spawn_ax88179() {
    use crate::hardware::ax88179_capsule as c;
    let spawn = c::spawn_driver_ax88179_capsule;
    super::super::boot::capsule("DRIVER-AX88179", "driver_ax88179", spawn, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-ax88179"))]
fn spawn_ax88179() {}

#[cfg(feature = "nonos-capsule-driver-rtl8153")]
fn spawn_rtl8153() {
    use crate::hardware::rtl8153_capsule as c;
    let spawn = c::spawn_driver_rtl8153_capsule;
    super::super::boot::capsule("DRIVER-RTL8153", "driver_rtl8153", spawn, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-rtl8153"))]
fn spawn_rtl8153() {}
