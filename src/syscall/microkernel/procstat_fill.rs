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

//! Filling one `ProcStatEntry` from the process control block and the
//! accounting counters, all read once under the block's own locks.

use core::sync::atomic::Ordering;

use super::procstat_codes::{priority_code, state_code};
use super::procstat_entry::{ProcStatEntry, PROC_NAME_LEN};
use crate::process::accounting;
use crate::process::scheduler::preemption::proc_ticks;

/// Everything straight from the process control block and the counters.
pub(super) fn entry_for(pid: u32, now_ms: u64) -> ProcStatEntry {
    let c = accounting::snapshot(pid);
    let mut e = ProcStatEntry {
        pid,
        state: 0,
        name_len: 0,
        priority: 0,
        _pad: 0,
        run_ticks: proc_ticks::ticks_for(pid),
        caps: 0,
        mem_kb: 0,
        uptime_ms: 0,
        name: [0; PROC_NAME_LEN],
        ppid: 0,
        _pad2: 0,
        syscalls: c.syscalls,
        ipc_tx: c.ipc_tx,
        ipc_rx: c.ipc_rx,
        faults: c.faults,
        switches: c.switches,
        user_ticks: c.user_ticks,
        mapped_kb: 0,
        vma_count: 0,
        _pad3: 0,
    };
    crate::process::with_process(pid, |pcb| {
        e.state = state_code(&pcb.state.lock());
        e.priority = priority_code(&pcb.priority.lock());
        e.caps = pcb.caps_bits.load(Ordering::Relaxed);
        let mem = pcb.memory.lock();
        e.mem_kb = mem.resident_pages.load(Ordering::Relaxed) as u64 * 4;
        e.vma_count = mem.vmas.len() as u32;
        e.mapped_kb =
            mem.vmas.iter().map(|v| v.end.as_u64().saturating_sub(v.start.as_u64())).sum::<u64>()
                / 1024;
        drop(mem);
        e.uptime_ms = now_ms.saturating_sub(pcb.start_time_ms.load(Ordering::Relaxed));
        e.ppid = pcb.parent_pid();
        let name = pcb.name.lock();
        let n = name.len().min(PROC_NAME_LEN);
        e.name[..n].copy_from_slice(&name.as_bytes()[..n]);
        e.name_len = n as u8;
    });
    e
}
