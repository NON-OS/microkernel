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

/*
 * The apps a boot profile leaves off, on top of what setup chose. The
 * kernel's spawn gate already keeps every network driver and service from
 * starting on these boots; this keeps the dock honest about what can run.
 * Safe Mode runs no optional app, Recovery only Files and the text editor
 * beside the Terminal and Settings every boot has, Air-Gapped no Browser
 * and no store, which have nothing to reach.
 */

use super::bits::{BROWSER, EDITOR, FILES, STORE};
use crate::boot::handoff::{boot_profile, BootProfile};

pub(crate) fn apply() {
    let profile = boot_profile();
    for part in [b"[INIT] boot profile: ".as_slice(), profile.name().as_bytes(), b"\n"] {
        crate::sys::serial::print(part);
    }
    super::gate::withhold(withheld(profile));
}

fn withheld(profile: BootProfile) -> u32 {
    match profile {
        BootProfile::Standard | BootProfile::Hardened => 0,
        BootProfile::AirGapped => BROWSER | STORE,
        BootProfile::Safe => u32::MAX,
        BootProfile::Recovery => !(FILES | EDITOR),
    }
}
