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
//! The spawn path's hand-off to a terminal run: its run request goes to the
//! process, and it is recorded as a private run, before it first runs.

use super::super::roles::TERMINAL;

/// Called by the spawn path for every capsule, after the last step that can
/// fail and before the process is queued to run. For the terminal-run role
/// named `name` it hands `pid` the request its slot was reserved with and
/// records the slot held, so the process is private from its first
/// instruction. Any other name is left alone.
pub fn admit_terminal_run(name: &str, pid: u32) {
    let Some(i) = TERMINAL.iter().position(|r| r.name == name) else {
        return;
    };
    match super::slots::hold(i, pid) {
        Some(argv) => {
            crate::process::with_process(pid, |pcb| *pcb.argv.lock() = argv);
        }
        /*
         * Only `run_tier_for_caller` spawns these roles, always into a slot
         * it reserved. Without a request the process runs with no arguments,
         * which the personality refuses; say so.
         */
        None => crate::sys::serial::print(b"[LINUX-TERM] admitted with no reserved slot\n"),
    }
}

/// Whether `pid` belongs to a terminal run, whose every byte of output is
/// private: the run itself, a thread of it, or a guest it hosts (or a
/// thread of one).
pub fn is_private_run(pid: u32) -> bool {
    let run = |p: u32| super::held::holds(p) || super::held::holds(group_of(p));
    run(pid)
        || [pid, group_of(pid)]
            .into_iter()
            .filter_map(crate::process::foreign::supervisor_of)
            .any(run)
}

fn group_of(pid: u32) -> u32 {
    crate::process::with_process(pid, |pcb| pcb.thread_group_id()).unwrap_or(pid)
}
