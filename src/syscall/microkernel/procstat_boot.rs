/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/* How this boot was started, as `MkProcStat` reports it to programs. */

use super::procstat_header::{
    BOOT_INSTALL_REQUESTED, BOOT_PROFILE_AIR_GAPPED, BOOT_PROFILE_HARDENED, BOOT_PROFILE_RECOVERY,
    BOOT_PROFILE_SAFE,
};
use crate::boot::handoff::{boot_profile, install_requested, BootProfile};

pub(super) fn flags() -> u32 {
    let install = if install_requested() { BOOT_INSTALL_REQUESTED } else { 0 };
    let profile = match boot_profile() {
        BootProfile::Standard => 0,
        BootProfile::Hardened => BOOT_PROFILE_HARDENED,
        BootProfile::Safe => BOOT_PROFILE_SAFE,
        BootProfile::AirGapped => BOOT_PROFILE_AIR_GAPPED,
        BootProfile::Recovery => BOOT_PROFILE_RECOVERY,
    };
    install | profile
}
