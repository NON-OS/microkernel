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

extern crate alloc;

use crate::smp::MAX_CPUS;

use super::percpu::PerCpuLog;

#[derive(Debug, Clone)]
pub struct SyscallAuditEntry {
    pub timestamp_ms: u64,
    pub syscall_num: u64,
    pub syscall_name: &'static str,
    pub pid: u32,
    pub result: i64,
    pub args: [u64; 4],
    pub success: bool,
}

pub(super) static AUDIT_LOG: PerCpuLog<SyscallAuditEntry, MAX_CPUS> = PerCpuLog::new();

/// The `max_entries` most recent records, newest first.
pub fn get_audit_log(max_entries: usize) -> alloc::vec::Vec<SyscallAuditEntry> {
    AUDIT_LOG.recent(max_entries)
}

pub fn clear_audit_log() {
    AUDIT_LOG.clear();
}
