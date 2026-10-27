// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The MediaTek MT7921/MT7922 driver, started only when the boot PCI scan
//! found one of its parts. Without the card a spawn would cost an
//! attestation the boot waits for, and the driver would only say it could
//! not claim one.

#[cfg(feature = "nonos-capsule-driver-mt7921")]
pub(super) fn spawn() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::mt7921_capsule as c;
    if !super::super::device_present::present("DRIVER-MT7921", HardwareFamily::NetworkMt7921) {
        return;
    }
    super::super::boot::capsule(
        "DRIVER-MT7921",
        "driver_mt7921",
        c::spawn_driver_mt7921_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-mt7921"))]
pub(super) fn spawn() {}
