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

//! Whether a process a CPU is about to run, or is running, has been killed.
//!
//! On one CPU a process is only ever torn down by itself or while it is off
//! the processor. With several, another CPU can kill it while it runs here,
//! or between the claim that picked it and the switch into it. The switch
//! and the preemption paths ask this so they never resume such a process.

use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};

/// True when `pid` is torn down or gone from the table. Always false on the
/// single-CPU image, where the question cannot arise.
pub(crate) fn is_dead(pid: u32) -> bool {
    if !super::on_cpu::TRACKED || pid == 0 {
        return false;
    }
    match PROCESS_TABLE.find_by_pid(pid) {
        Some(pcb) => {
            matches!(*pcb.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_))
        }
        None => true,
    }
}
