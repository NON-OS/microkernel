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

//! Whether a thread group's devices may go yet (L77). A thread killed from
//! another CPU runs on there until that CPU takes the stop IPI; with its DMA
//! frames freed and its claims ended under it, it would reach frames handed
//! to someone else and a device given to its next claimant. So the release
//! waits until no CPU but this one is on the tables of any of the group's
//! threads; past the bound it is left to finalize, which runs only then.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{fence, Ordering};

use crate::hardware::broker::holder_of;
use crate::process::core::PROCESS_TABLE;
use crate::process::scheduler::selection::cpu_on_tables;

/// How long a teardown waits for them (our bound): a CPU in user code leaves
/// at the IPI, one in a call at the call's end.
const WAIT_NS: u64 = 10_000_000;

/// True once none of `holder`'s threads is on another CPU; `wait` spins for
/// that up to the bound, answering shootdowns, else it is asked once.
pub(super) fn group_off_cpu(holder: u32, wait: bool) -> bool {
    let group: Vec<u32> = PROCESS_TABLE
        .get_all_processes()
        .into_iter()
        .map(|p| p.pid)
        .filter(|&pid| holder_of(pid) == holder)
        .collect();
    let me = crate::smp::cpu_id();
    let start = crate::time::now_ns();
    loop {
        // Pairs with the fence in the switch: a CPU that names a thread
        // before seeing it dead is seen here (see `switch_to_process`).
        fence(Ordering::SeqCst);
        if !group.iter().any(|&pid| cpu_on_tables(pid).is_some_and(|cpu| cpu != me)) {
            return true;
        }
        if !wait || crate::time::now_ns().saturating_sub(start) >= WAIT_NS {
            return false;
        }
        crate::smp::serve_shootdowns();
        core::hint::spin_loop();
    }
}
