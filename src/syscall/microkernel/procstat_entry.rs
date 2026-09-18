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

//! One process as `MkProcStat` reports it. The layout is the wire format
//! `nonos_libc::ProcStatEntry` mirrors field for field; new fields are
//! appended, never inserted, so an older reader keeps its offsets.

/// Name bytes carried inline so a monitor needs no extra lookup per pid.
pub const PROC_NAME_LEN: usize = 24;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcStatEntry {
    pub pid: u32,
    /// 0 new, 1 ready, 2 running, 3 sleeping, 4 stopped, 5 zombie, 6 terminated.
    pub state: u8,
    pub name_len: u8,
    /// Scheduling class: 0 idle, 1 low, 2 normal, 3 high, 4 realtime.
    pub priority: u8,
    pub _pad: u8,
    pub run_ticks: u64,
    pub caps: u64,
    pub mem_kb: u64,
    /// Milliseconds alive on the same monotonic clock the header uses.
    pub uptime_ms: u64,
    pub name: [u8; PROC_NAME_LEN],
    pub ppid: u32,
    pub _pad2: u32,
    pub syscalls: u64,
    pub ipc_tx: u64,
    pub ipc_rx: u64,
    pub faults: u64,
    pub switches: u64,
    /// Ticks that interrupted this process's own code; the rest of
    /// `run_ticks` was the kernel working for it.
    pub user_ticks: u64,
    /// Every mapped region summed, and how many there are.
    pub mapped_kb: u64,
    pub vma_count: u32,
    pub _pad3: u32,
}
