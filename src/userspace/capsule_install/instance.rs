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

//! The installer starts when a person opens it and never at boot. One
//! window at a time: a click while it is running focuses the window it has,
//! and a click after it exited starts a fresh one. Liveness is read from the
//! process table rather than from the recorded pid alone, because the pid a
//! spawn recorded stays recorded after the capsule exits.

use super::spawn::spawn_install_capsule;
use super::state;
use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

pub fn spawn_install_instance() -> Result<u32, SpawnError> {
    let st = state::shared_state();
    let pid = st.pid();
    if st.is_alive() && crate::process::core::PROCESS_TABLE.find_by_pid(pid).is_some() {
        return Ok(pid);
    }
    spawn_install_capsule()
}
