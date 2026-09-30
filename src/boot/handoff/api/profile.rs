/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The boot profile the person chose in the boot menu, as the loader passed
 * it. The loader already ran that profile's checks; this is what the kernel
 * does differently once it runs. A kernel with no handoff runs Standard.
 */

use super::super::types::flags;
use super::query::get_handoff;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BootProfile {
    Standard,
    Hardened,
    Safe,
    AirGapped,
    Recovery,
}

impl BootProfile {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Standard => "Standard",
            Self::Hardened => "Hardened",
            Self::Safe => "Safe Mode",
            Self::AirGapped => "Air-Gapped",
            Self::Recovery => "Recovery",
        }
    }

    /* Whether any network driver or network service may start. */
    pub const fn network(self) -> bool {
        matches!(self, Self::Standard | Self::Hardened)
    }

    /* Safe Mode starts no audio driver and no optional app. */
    pub const fn minimal(self) -> bool {
        matches!(self, Self::Safe)
    }

    /* Recovery goes straight to its desktop; setup does not run. */
    pub const fn skips_setup(self) -> bool {
        matches!(self, Self::Recovery)
    }
}

pub fn boot_profile() -> BootProfile {
    let Some(h) = get_handoff() else {
        return BootProfile::Standard;
    };
    if h.has_flag(flags::PROFILE_RECOVERY) {
        BootProfile::Recovery
    } else if h.has_flag(flags::PROFILE_SAFE) {
        BootProfile::Safe
    } else if h.has_flag(flags::PROFILE_AIR_GAPPED) {
        BootProfile::AirGapped
    } else if h.has_flag(flags::PROFILE_HARDENED) {
        BootProfile::Hardened
    } else {
        BootProfile::Standard
    }
}
