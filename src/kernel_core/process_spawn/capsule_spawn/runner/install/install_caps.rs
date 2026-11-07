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

use super::super::super::spec::SpawnError;
use crate::process::caps as proc_caps;

pub(super) fn install_caps(pid: u32, caps_bits: u64, envelope: u64) -> Result<(), SpawnError> {
    /* Every spawn path installs its caps here; the boot profile trims them. */
    let caps = super::super::profile_gate::caps(caps_bits);
    proc_caps::install_spawn(pid, caps).ok_or(SpawnError::ProcessCreation)?;
    /*
     * The grant bound, trimmed by the same profile so no grant hands back a
     * bit this boot withheld; the installed word is inside it by construction.
     */
    let bound = super::super::profile_gate::caps(envelope) | caps;
    crate::process::caps_envelope::record(pid, bound).ok_or(SpawnError::ProcessCreation)?;
    // Untrimmed: what the manifest declares, the line a call may not cross.
    crate::process::caps::declare(pid, envelope).ok_or(SpawnError::ProcessCreation)?;
    // Said in the process table, so a withheld bit never reads as a manifest change.
    if super::super::profile_gate::withholds_debug(caps_bits) {
        proc_caps::mark_debug_withheld(pid);
    }
    Ok(())
}
