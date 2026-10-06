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

//! The shell's own children, as the process table reports them. The installer
//! makes the shell the parent of an app it loads for it, so a child that was
//! not there before a launch, and is no older than the launch, is that app,
//! whatever service name its manifest gave it. Only the pid, parent and age
//! of each entry are read, which the table shows any caller.

use alloc::vec;
use alloc::vec::Vec;
use core::mem::size_of;

use nonos_libc::{mk_getpid, mk_proc_stat, ProcStatEntry, ProcStatHeader};

/// Entries read at most, so a crowded table cannot make one launch allocate
/// without bound.
const MAX_ENTRIES: usize = 512;
/// Slack on the age test for the time between the load and the first look.
const AGE_SLACK_MS: u64 = 1_000;

fn table() -> Vec<ProcStatEntry> {
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

/// The pids of the shell's children now.
pub(super) fn children() -> Vec<u32> {
    let me = mk_getpid();
    table().iter().filter(|e| e.ppid == me && e.pid != 0).map(|e| e.pid).collect()
}

/// A child born since `before` was taken and within `age_ms`, the youngest
/// if there are several.
pub(super) fn new_child(before: &[u32], age_ms: i64) -> Option<u32> {
    let me = mk_getpid();
    let limit = (age_ms.max(0) as u64).saturating_add(AGE_SLACK_MS);
    table()
        .iter()
        .filter(|e| e.ppid == me && e.pid != 0 && !before.contains(&e.pid))
        .filter(|e| e.uptime_ms <= limit)
        .min_by_key(|e| e.uptime_ms)
        .map(|e| e.pid)
}
