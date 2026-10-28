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

//! The ASIX AX88179/AX88178A driver, apart from the class drivers so each
//! file stays short; spawned, as they are, wherever a USB host is.

#[cfg(feature = "nonos-capsule-driver-ax88179")]
pub(super) fn spawn() {
    use crate::hardware::ax88179_capsule as c;
    let spawn = c::spawn_driver_ax88179_capsule;
    super::super::boot::capsule("DRIVER-AX88179", "driver_ax88179", spawn, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-ax88179"))]
pub(super) fn spawn() {}
