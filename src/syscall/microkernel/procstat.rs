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

//! `MkProcStat` copies a `ProcStatHeader` plus one `ProcStatEntry` per
//! live pid; a NULL buffer or zero `max_entries` probes the pid count.
//! The header is the machine (`procstat_header.rs`), an entry is one
//! process (`procstat_entry.rs`); both are the kernel's own counters, so a
//! monitor built on them shows what the kernel did and nothing it guessed.

use core::mem::size_of;

use super::errnos::ERRNO_FAULT;
use super::procstat_entry::ProcStatEntry;
use super::procstat_fill::entry_for;
use super::procstat_header::ProcStatHeader;
use super::procstat_header_fill::header_for;
use crate::usercopy::{validate_user_write, write_user_value};

pub use super::procstat_entry::PROC_NAME_LEN;

pub fn sys_proc_stat(buf_ptr: u64, max_entries: u64) -> i64 {
    let pids = crate::process::list_all_pids();
    if max_entries == 0 || buf_ptr == 0 {
        return pids.len() as i64;
    }
    let to_write = core::cmp::min(max_entries as usize, pids.len());
    let bytes = size_of::<ProcStatHeader>() + to_write * size_of::<ProcStatEntry>();
    if validate_user_write(buf_ptr, bytes).is_err() {
        return ERRNO_FAULT;
    }
    let now_ms = crate::time::timestamp_millis();
    if write_user_value(buf_ptr, &header_for(to_write as u32, now_ms)).is_err() {
        return ERRNO_FAULT;
    }
    let mut dst = buf_ptr + size_of::<ProcStatHeader>() as u64;
    for pid in pids.iter().take(to_write) {
        if write_user_value(dst, &entry_for(*pid, now_ms)).is_err() {
            return ERRNO_FAULT;
        }
        dst += size_of::<ProcStatEntry>() as u64;
    }
    to_write as i64
}
