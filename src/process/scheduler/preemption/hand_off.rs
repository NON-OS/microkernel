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

//! The steps a preempted and a yielding process share around a switch.

use super::super::dispatch::add_to_run_queue;
use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};

/// A Running `pid` becomes Ready, and a Ready one goes back on the run queue.
pub(super) fn requeue(pid: u32) {
    let runnable = if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        let mut state = pcb.state.lock();
        if matches!(*state, ProcessState::Running) {
            *state = ProcessState::Ready;
        }
        matches!(*state, ProcessState::Ready)
    } else {
        false
    };
    if runnable {
        add_to_run_queue(pid);
    }
}

/// Nothing else was picked: a Ready `pid` runs on.
pub(super) fn run_on(pid: u32) {
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        let mut state = pcb.state.lock();
        if matches!(*state, ProcessState::Ready) {
            *state = ProcessState::Running;
        }
    }
}

/// Count a switch to `next`.
pub(super) fn count_switch(next: u32) {
    crate::process::accounting::bump(next, crate::process::accounting::Kind::Switch);
    crate::process::accounting::bump_total(crate::process::accounting::Total::Switches);
}
