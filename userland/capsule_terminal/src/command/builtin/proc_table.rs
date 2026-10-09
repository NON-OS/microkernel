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

//! The kernel's process table, as `MkProcStat` hands it to this terminal:
//! every process's pid, parent, name and state, which any caller may read.

use alloc::vec;
use alloc::vec::Vec;
use core::mem::size_of;

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

/// Entries read at most, so a crowded table cannot make one call allocate
/// without bound.
const MAX_ENTRIES: usize = 512;

/// Every process the table lists, or nothing when the kernel refused.
pub fn table() -> Vec<ProcStatEntry> {
    let count = mk_proc_stat(core::ptr::null_mut(), 0);
    if count <= 0 {
        return Vec::new();
    }
    let max = (count as usize).saturating_add(8).min(MAX_ENTRIES);
    let mut buf = vec![0u8; size_of::<ProcStatHeader>() + max * size_of::<ProcStatEntry>()];
    let got = mk_proc_stat(buf.as_mut_ptr(), max as u32);
    if got <= 0 {
        return Vec::new();
    }
    let got = (got as usize).min(max);
    let mut out = Vec::with_capacity(got);
    for i in 0..got {
        let at = size_of::<ProcStatHeader>() + i * size_of::<ProcStatEntry>();
        // In bounds by the buffer's size above; read unaligned, as the
        // buffer is bytes.
        let e = unsafe { core::ptr::read_unaligned(buf[at..].as_ptr() as *const ProcStatEntry) };
        out.push(e);
    }
    out
}

/// The scheduler's state code as a word, as the process monitor says it: a
/// capsule parked on IPC is alive and answers at once, so it is "idle".
pub fn state_word(state: u8) -> &'static [u8] {
    match state {
        0 => b"new",
        1 => b"ready",
        2 => b"running",
        3 => b"idle",
        4 => b"stopped",
        5 => b"zombie",
        _ => b"exited",
    }
}
