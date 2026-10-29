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

//! The Bluetooth LE host, at boot, wherever an xHCI host is: the only host
//! it reaches its controller through. Whether a Bluetooth controller is on
//! that host's ports is known only once the capsule looks, as with every
//! USB class driver: the kernel enumerates no USB device. Without one it
//! says `no controller yet` once and looks again each second. With one,
//! its scan is passive: nothing of this host goes on the air until it
//! connects to a bonded device, or to a new one in a pairing window
//! Settings opened (NBTC's PAIR).

#[cfg(feature = "nonos-capsule-driver-bt-usb")]
pub(super) fn spawn() {
    use crate::hardware::bt_usb_capsule as c;
    use crate::hardware::inventory::{present, HardwareFamily};
    if !present(HardwareFamily::UsbXhci) {
        crate::sys::boot_log::ok("DRIVER-BT-USB", "no xHCI host present, not spawned");
        return;
    }
    let spawn = c::spawn_driver_bt_usb_capsule;
    super::super::boot::capsule("DRIVER-BT-USB", "driver_bt_usb", spawn, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-driver-bt-usb"))]
pub(super) fn spawn() {}
