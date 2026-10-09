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

use super::on_cpu::held_elsewhere;

/// Take `pid` from `Ready` to `Running` under the state lock, reporting
/// whether this caller made the transition. A pid another CPU is still
/// running on, or still leaving, is `Ready` only on paper: its stack is in
/// use, so it is refused until that CPU is off it (see `on_cpu`).
pub(super) fn claim(pid: u32) -> bool {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) else {
        return false;
    };
    let mut state = pcb.state.lock();
    if *state == ProcessState::Ready && !held_elsewhere(pid) {
        *state = ProcessState::Running;
        true
    } else {
        false
    }
}
