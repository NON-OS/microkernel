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

//! The USB hub walker, after the hosts and before the class drivers, so
//! the devices below hubs are parked as virtual ports by the time the class
//! drivers search. It starts wherever a USB host is.

#[cfg(feature = "nonos-capsule-driver-usb-hub")]
pub(super) fn spawn() {
    use crate::hardware::usb_hub_capsule as c;
    if !super::usb_host::usb_host() {
        crate::sys::boot_log::ok("DRIVER-USB-HUB", "no USB host present, not spawned");
        return;
    }
    let spawn = c::spawn_driver_usb_hub_capsule;
    super::boot::capsule("DRIVER-USB-HUB", "driver_usb_hub", spawn, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-driver-usb-hub"))]
pub(super) fn spawn() {}
