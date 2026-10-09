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

/* The memory this machine has, as the kernel counts it. */

use core::mem::size_of;

use alloc::vec;

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

/* Bytes of memory; `None` when the kernel would not say. */
pub fn memory() -> Option<u64> {
    let mut buf = vec![0u8; size_of::<ProcStatHeader>() + size_of::<ProcStatEntry>()];
    if mk_proc_stat(buf.as_mut_ptr(), 1) < 0 {
        return None;
    }
    /*
     * The kernel wrote a whole header at the start of the buffer, which is
     * as long as one; it is read unaligned, as the Terminal reads it.
     */
    let header = unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const ProcStatHeader) };
    (header.mem_total_kb > 0).then(|| header.mem_total_kb * 1024)
}
