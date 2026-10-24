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

//! The USB smart-card reader driver, after the hub walker, so a reader
//! below a hub is parked as a virtual port by the time it searches. It
//! starts wherever a USB host is.

#[cfg(feature = "nonos-capsule-driver-ccid")]
pub(super) fn spawn() {
    use crate::hardware::ccid_capsule as c;
    if !super::usb_host::usb_host() {
        crate::sys::boot_log::ok("DRIVER-CCID", "no USB host present, not spawned");
        return;
    }
    let spawn = c::spawn_driver_ccid_capsule;
    super::boot::capsule("DRIVER-CCID", "driver_ccid", spawn, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-driver-ccid"))]
pub(super) fn spawn() {}
