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

//! Who a pid is, as the kernel names it.

use core::mem::size_of;

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader, PROC_NAME_LEN};

use super::types::MAX_PROCS;

const HEADER_LEN: usize = size_of::<ProcStatHeader>();
const ENTRY_LEN: usize = size_of::<ProcStatEntry>();

/// The spawn name and capability mask of `pid`, or `None` when the kernel does
/// not answer or the pid is not running. This capsule holds AttestRead, so the
/// kernel shows it every process's mask rather than the redacted zero.
pub fn identity(pid: u32) -> Option<([u8; PROC_NAME_LEN], usize, u64)> {
    if pid == 0 {
        return None;
    }
    let mut buf = [0u8; HEADER_LEN + MAX_PROCS * ENTRY_LEN];
    let written = mk_proc_stat(buf.as_mut_ptr(), MAX_PROCS as u32);
    if written <= 0 {
        return None;
    }
    let count = (written as usize).min(MAX_PROCS);
    for i in 0..count {
        let off = HEADER_LEN + i * ENTRY_LEN;
        if off + ENTRY_LEN > buf.len() {
            break;
        }
        // SAFETY: eK@nonos.systems - `off + ENTRY_LEN` is inside `buf`, checked
        // above, and the entry is plain data read unaligned.
        let e: ProcStatEntry =
            unsafe { core::ptr::read_unaligned(buf.as_ptr().add(off) as *const ProcStatEntry) };
        if e.pid == pid {
            let n = (e.name_len as usize).min(PROC_NAME_LEN);
            return Some((e.name, n, e.caps));
        }
    }
    None
}
