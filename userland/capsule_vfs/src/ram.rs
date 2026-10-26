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

//! The machine's memory, as the kernel's process table states it.

use core::mem::size_of;

use nonos_libc::procstat::PROC_STAT_VERSION;
use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

/// Bytes of memory the machine has, or None when the kernel does not say.
pub fn total() -> Option<u64> {
    let mut buf = [0u8; size_of::<ProcStatHeader>() + size_of::<ProcStatEntry>()];
    if mk_proc_stat(buf.as_mut_ptr(), 1) < 0 {
        return None;
    }
    /*
     * SAFETY: `buf` holds a whole header, and every bit pattern is a valid
     * ProcStatHeader, which is plain integers.
     */
    let h: ProcStatHeader = unsafe { core::ptr::read_unaligned(buf.as_ptr().cast()) };
    (h.version >= PROC_STAT_VERSION && h.mem_total_kb > 0).then(|| h.mem_total_kb * 1024)
}
