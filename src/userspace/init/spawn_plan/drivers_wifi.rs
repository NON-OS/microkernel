/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The Wi-Fi drivers, each started only when the boot PCI scan found its
//! card. Without the card the iwlwifi driver exited at once and the
//! RTL8821CE driver stayed resident answering that it could not claim one;
//! either way the spawn cost an attestation the boot waited for.

#[cfg(any(
    feature = "nonos-capsule-driver-iwlwifi",
    feature = "nonos-capsule-driver-rtl8821ce"
))]
use crate::hardware::inventory::HardwareFamily;

pub(super) fn spawn() {
    spawn_iwlwifi();
    spawn_rtl8821ce();
}

#[cfg(feature = "nonos-capsule-driver-iwlwifi")]
fn spawn_iwlwifi() {
    use crate::hardware::iwlwifi_capsule as c;
    if !super::device_present::present("DRIVER-IWLWIFI", HardwareFamily::NetworkIwlwifi) {
        return;
    }
    super::boot::capsule(
        "DRIVER-IWLWIFI",
        "driver_iwlwifi",
        c::spawn_driver_iwlwifi_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-iwlwifi"))]
fn spawn_iwlwifi() {}

#[cfg(feature = "nonos-capsule-driver-rtl8821ce")]
fn spawn_rtl8821ce() {
    use crate::hardware::rtl8821ce_capsule as c;
    if !super::device_present::present("DRIVER-RTL8821CE", HardwareFamily::NetworkRtl8821ce) {
        return;
    }
    super::boot::capsule(
        "DRIVER-RTL8821CE",
        "driver_rtl8821ce",
        c::spawn_driver_rtl8821ce_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-rtl8821ce"))]
fn spawn_rtl8821ce() {}
