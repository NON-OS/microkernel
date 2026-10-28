// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The RTL8852BE driver, in a file of its own so the Wi-Fi spawns stay
//! short; started, as the RTL8821CE's is, only when the scan found its card.

#[cfg(feature = "nonos-capsule-driver-rtl8852be")]
pub(super) fn spawn() {
    use crate::hardware::inventory::HardwareFamily::NetworkRtl8852be;
    use crate::hardware::rtl8852be_capsule as c;
    if !super::super::device_present::present("DRIVER-RTL8852BE", NetworkRtl8852be) {
        return;
    }
    let spawn = c::spawn_driver_rtl8852be_capsule;
    super::super::boot::capsule("DRIVER-RTL8852BE", "driver_rtl8852be", spawn, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-rtl8852be"))]
pub(super) fn spawn() {}
