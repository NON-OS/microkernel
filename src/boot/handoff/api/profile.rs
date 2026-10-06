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

//! The boot profile chosen in the boot menu, as the loader passed it. The
//! loader ran that profile's checks; this is what the kernel does differently.
//! A kernel with no handoff runs Standard.

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
