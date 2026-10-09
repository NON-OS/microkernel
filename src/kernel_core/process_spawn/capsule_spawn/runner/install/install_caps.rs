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

pub(super) fn install_caps(pid: u32, caps_bits: u64) -> Result<(), SpawnError> {
    /* Every spawn path installs its caps here; the boot profile trims them. */
    let caps = super::super::profile_gate::caps(caps_bits);
    proc_caps::install_spawn(pid, caps).ok_or(SpawnError::ProcessCreation)
}
