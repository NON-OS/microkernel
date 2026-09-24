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

//! The process table as the kernel reports it: one header for the machine,
//! one entry per live process. Both mirror the kernel's wire layout field
//! for field (src/syscall/microkernel/procstat_header.rs and
//! procstat_entry.rs); fields are appended, never inserted.

use crate::syscall::{call_raw, N_MK_PROC_STAT};

/// Inline process name length; must match the kernel's PROC_NAME_LEN.
pub const PROC_NAME_LEN: usize = 24;

/// The header layout this crate was built for.
pub const PROC_STAT_VERSION: u32 = 3;

/// state: 0 new, 1 ready, 2 running, 3 sleeping, 4 stopped, 5 zombie,
/// 6 terminated. priority: 0 idle, 1 low, 2 normal, 3 high, 4 realtime.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ProcStatEntry {
    pub pid: u32,
    pub state: u8,
    pub name_len: u8,
    pub priority: u8,
    pub _pad: u8,
    pub run_ticks: u64,
    pub caps: u64,
    pub mem_kb: u64,
    pub uptime_ms: u64,
    pub name: [u8; PROC_NAME_LEN],
    pub ppid: u32,
    pub _pad2: u32,
    pub syscalls: u64,
    pub ipc_tx: u64,
    pub ipc_rx: u64,
    pub faults: u64,
    pub switches: u64,
    pub user_ticks: u64,
    pub mapped_kb: u64,
    pub vma_count: u32,
    pub _pad3: u32,
}

impl ProcStatEntry {
    pub fn name_str(&self) -> &str {
        let n = (self.name_len as usize).min(PROC_NAME_LEN);
        core::str::from_utf8(&self.name[..n]).unwrap_or("")
    }
}

pub extern "C" fn mk_proc_stat(buf: *mut u8, max_entries: u32) -> i64 {
    call_raw(N_MK_PROC_STAT, [buf as u64, max_entries as u64, 0, 0, 0, 0])
}
