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
//! MkProcStat with no process table: it counts no rows and writes none, so
//! the pool reads no pid as ready.

#[path = "../../../libc/src/procstat_header.rs"]
pub mod procstat_header;
pub use procstat_header::ProcStatHeader;

/// The start of a row, as far as the pool reads one; none is written.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ProcStatEntry {
    pub pid: u32,
    pub state: u8,
    pub uptime_ms: u64,
}

pub extern "C" fn mk_proc_stat(_buf: *mut u8, _max_entries: u32) -> i64 {
    0
}
