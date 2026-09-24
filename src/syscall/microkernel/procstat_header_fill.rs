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

//! Filling the `ProcStatHeader` from the accounting totals, the allocators
//! and the scheduler, at the moment of the call.

use super::procstat_header::{ProcStatHeader, PROC_STAT_VERSION};
use crate::process::accounting;

pub(super) fn header_for(count: u32, now_ms: u64) -> ProcStatHeader {
    let totals = accounting::totals();
    let heap = crate::memory::heap::get_heap_stats();
    let free_frames = crate::memory::phys::allocator::phys_total_free_frames() as u64;
    ProcStatHeader {
        total_ticks: crate::interrupts::timer::state::get_ticks(),
        count,
        version: PROC_STAT_VERSION,
        mem_total_kb: crate::memory::phys::allocator::phys_total_memory() / 1024,
        load_avg_fixed: crate::fs::procfs::load_averages_fixed(),
        idle_ticks: accounting::idle_ticks(),
        mem_free_kb: free_frames * 4,
        heap_used_kb: heap.current_usage as u64 / 1024,
        heap_total_kb: heap.total_size as u64 / 1024,
        uptime_ms: now_ms,
        context_switches: totals.switches,
        syscalls: totals.syscalls,
        ipc_messages: totals.ipc_messages,
        interrupts: totals.interrupts,
        faults: totals.faults,
        cpus_online: crate::smp::cpus_online() as u32,
        _pad: 0,
        user_ticks: totals.user_ticks,
        kernel_ticks: totals.kernel_ticks,
        largest_free_kb: crate::memory::phys::allocator::phys_largest_free_run() as u64 * 4,
        heap_peak_kb: heap.peak_usage as u64 / 1024,
        heap_allocs: heap.allocation_count as u64,
    }
}
