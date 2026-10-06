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
