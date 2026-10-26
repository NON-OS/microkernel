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

//! MkProcStat over the fixed machine: a null buffer counts the processes,
//! a buffer gets the header and up to `max` rows.

use crate::machine::{name_of, PROCS};
pub use crate::proc_types::{ProcStatEntry, PROC_FLAG_DEBUG_WITHHELD, PROC_NAME_LEN};
pub use crate::procstat_header::ProcStatHeader;

const GIB_KB: u64 = 1024 * 1024;

fn header() -> ProcStatHeader {
    ProcStatHeader {
        count: PROCS.len() as u32,
        version: 4,
        mem_total_kb: 4 * GIB_KB,
        mem_free_kb: 3 * GIB_KB + 211_000,
        load_avg_fixed: [430, 380, 300],
        uptime_ms: 8_049_000,
        cpus_online: 4,
        ..ProcStatHeader::default()
    }
}

fn entry(i: usize) -> ProcStatEntry {
    let (pid, name, caps) = PROCS[i];
    let name = name_of(name).as_bytes();
    let mut e = ProcStatEntry { pid, state: 3, priority: 2, caps, ..ProcStatEntry::default() };
    e.name_len = name.len().min(PROC_NAME_LEN) as u8;
    e.name[..e.name_len as usize].copy_from_slice(&name[..e.name_len as usize]);
    e.mem_kb = 2048 + 512 * i as u64;
    e
}

pub fn mk_proc_stat(buf: *mut u8, max: u32) -> i64 {
    if buf.is_null() {
        return PROCS.len() as i64;
    }
    let rows = (max as usize).min(PROCS.len());
    // SAFETY: the caller hands room for a header and `max` rows, as the
    // real call requires; both are plain integers written unaligned.
    let (head, row) = (size_of::<ProcStatHeader>(), size_of::<ProcStatEntry>());
    unsafe {
        crate::user_mem::put(buf as usize, header());
        (0..rows).for_each(|i| crate::user_mem::put(buf as usize + head + i * row, entry(i)));
    }
    rows as i64
}
