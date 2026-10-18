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

//! The USB network class drivers, wherever a USB host is: whether an
//! adapter or a tethered phone is plugged in is known only once each driver
//! searches the ports, and each keeps searching while it has no device.

pub(super) fn spawn() {
    // Said only by a build that carries one of them.
    #[cfg(any(
        feature = "nonos-capsule-driver-cdc-ecm",
        feature = "nonos-capsule-driver-cdc-ncm",
        feature = "nonos-capsule-driver-rndis"
    ))]
    if !super::usb_host::usb_host() {
        crate::sys::boot_log::ok("DRIVER-USBNET", "no USB host, not spawned");
        return;
    }
    spawn_cdc_ecm();
    spawn_cdc_ncm();
    spawn_rndis();
}

#[cfg(feature = "nonos-capsule-driver-cdc-ecm")]
fn spawn_cdc_ecm() {
    use crate::hardware::cdc_ecm_capsule as c;
    let spawn = c::spawn_driver_cdc_ecm_capsule;
    super::boot::capsule("DRIVER-CDC-ECM", "driver_cdc_ecm", spawn, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-cdc-ecm"))]
fn spawn_cdc_ecm() {}

#[cfg(feature = "nonos-capsule-driver-cdc-ncm")]
fn spawn_cdc_ncm() {
    use crate::hardware::cdc_ncm_capsule as c;
    let spawn = c::spawn_driver_cdc_ncm_capsule;
    super::boot::capsule("DRIVER-CDC-NCM", "driver_cdc_ncm", spawn, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-cdc-ncm"))]
fn spawn_cdc_ncm() {}

#[cfg(feature = "nonos-capsule-driver-rndis")]
fn spawn_rndis() {
    use crate::hardware::rndis_capsule as c;
    let spawn = c::spawn_driver_rndis_capsule;
    super::boot::capsule("DRIVER-RNDIS", "driver_rndis", spawn, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-rndis"))]
fn spawn_rndis() {}
