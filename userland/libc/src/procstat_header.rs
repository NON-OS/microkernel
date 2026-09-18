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

//! The machine as `MkProcStat` reports it, ahead of the entries. Mirrors
//! the kernel's procstat_header.rs field for field; appended, never
//! inserted. Load averages are Q11 fixed point: 2048 reads as 1.00.

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ProcStatHeader {
    pub total_ticks: u64,
    pub count: u32,
    pub version: u32,
    pub mem_total_kb: u64,
    /// 1/5/15-minute load averages in Q11 fixed point: 2048 reads as 1.00.
    pub load_avg_fixed: [u64; 3],
    pub idle_ticks: u64,
    pub mem_free_kb: u64,
    pub heap_used_kb: u64,
    pub heap_total_kb: u64,
    pub uptime_ms: u64,
    pub context_switches: u64,
    pub syscalls: u64,
    pub ipc_messages: u64,
    pub interrupts: u64,
    pub faults: u64,
    pub cpus_online: u32,
    pub _pad: u32,
    pub user_ticks: u64,
    pub kernel_ticks: u64,
    pub largest_free_kb: u64,
    pub heap_peak_kb: u64,
    pub heap_allocs: u64,
}
