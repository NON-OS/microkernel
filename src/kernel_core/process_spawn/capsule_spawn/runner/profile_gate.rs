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
 * The boot profile, enforced where every verified capsule starts. On an
 * Air-Gapped, Safe Mode or Recovery boot no network driver or service runs,
 * whoever asks, and every other program loses the Network capability, so
 * nothing may start one later. Safe Mode also starts no audio and no game.
 * Each refusal is on the serial log by name.
 */

use crate::boot::handoff::boot_profile;
use crate::capabilities::Capability;

use super::super::spec::SpawnError;
use super::profile_refuse::refused;

pub(in super::super) fn check(name: &str) -> Result<(), SpawnError> {
    let profile = boot_profile();
    if refused(profile, name) {
        for part in [
            b"[PROFILE] ".as_slice(),
            profile.name().as_bytes(),
            b": not started: ",
            name.as_bytes(),
            b"\n",
        ] {
            crate::sys::serial::print(part);
        }
        return Err(SpawnError::ProfileRefused);
    }
    Ok(())
}

/* The caps a capsule runs with under this boot's profile. */
pub(in super::super) fn caps(caps: u64) -> u64 {
    if boot_profile().network() {
        caps
    } else {
        caps & !Capability::Network.bit()
    }
}
