// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The RTL8822CE driver, in a file of its own so the Wi-Fi spawns stay
//! short; started, as the RTL8821CE's is, only when the scan found its card.

#[cfg(feature = "nonos-capsule-driver-rtl8822ce")]
pub(super) fn spawn() {
    use crate::hardware::inventory::HardwareFamily::NetworkRtl8822ce;
    use crate::hardware::rtl8822ce_capsule as c;
    if !super::super::device_present::present("DRIVER-RTL8822CE", NetworkRtl8822ce) {
        return;
    }
    let spawn = c::spawn_driver_rtl8822ce_capsule;
    super::super::boot::capsule("DRIVER-RTL8822CE", "driver_rtl8822ce", spawn, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-rtl8822ce"))]
pub(super) fn spawn() {}
