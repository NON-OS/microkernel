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

//! Which deferred kernel stacks a CPU may still be on.
//!
//! The originating core is not always the one that was on the stack: a
//! process killed from another CPU is queued on the killer's. So each entry
//! keeps its pid, and one that any CPU still runs or is leaving waits for a
//! later tick. The single-CPU image never finds one held.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{fence, Ordering};

use crate::process::core::Pid;
use crate::process::scheduler::selection::cpu_holding;

/// Take the stack tops no CPU still holds out of `q`, leaving the rest.
pub(super) fn take_unheld(q: &mut Vec<(Pid, u64)>) -> Vec<u64> {
    /*
     * Pairs with the fence in the switch: see `switch_to_process`.
     */
    fence(Ordering::SeqCst);
    let mut free = Vec::new();
    q.retain(|&(pid, top)| {
        let held = cpu_holding(pid).is_some();
        if !held {
            free.push(top);
        }
        held
    });
    free
}
