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
//! Whether a process lives, from the kernel's process table
//! (MkProcStat). Every capsule may read it, and a row's state is never
//! withheld. Read only while the compositor is unheard, so its cost is spent
//! rarely.

extern crate alloc;

use alloc::vec;
use core::mem::size_of;

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

const HEADER: usize = size_of::<ProcStatHeader>();
const ENTRY: usize = size_of::<ProcStatEntry>();

/// True while `pid` has a row that lives and is older than `quiet_ms`.
pub(super) fn lives(pid: u32, quiet_ms: u64) -> bool {
    let count = mk_proc_stat(core::ptr::null_mut(), 0);
    if count <= 0 {
        return false;
    }
    // Room for a few more, in case one starts between the two calls.
    let room = count as usize + 8;
    let mut buf = vec![0u8; HEADER + room * ENTRY];
    let written = mk_proc_stat(buf.as_mut_ptr(), room as u32).clamp(0, room as i64) as usize;
    (0..written).any(|i| {
        let raw = &buf[HEADER + i * ENTRY..HEADER + (i + 1) * ENTRY];
        // SAFETY: `raw` holds ENTRY bytes, and every bit pattern is a valid
        // ProcStatEntry, which is plain integers and bytes.
        let e: ProcStatEntry = unsafe { core::ptr::read_unaligned(raw.as_ptr().cast()) };
        e.pid == pid && nonos_gfx_serve::lives(e.state, e.uptime_ms, quiet_ms)
    })
}
