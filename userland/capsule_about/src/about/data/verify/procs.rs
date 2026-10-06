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

//! One read of the kernel process table, handed out an entry at a time, so
//! the tally and the names describe the same table the same way.

use alloc::vec;
use core::mem::size_of;
use core::ptr;

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

const HEADER_LEN: usize = size_of::<ProcStatHeader>();
const ENTRY_LEN: usize = size_of::<ProcStatEntry>();
/// Room for processes that start between the count and the read.
const SLACK: usize = 16;
/// Reads before giving up on a table that keeps outgrowing its buffer.
const TRIES: usize = 3;

/// How many processes are live, as the kernel counts them (`MkProcStat`
/// with no buffer); `None` when it would not answer.
pub(crate) fn live_count() -> Option<u32> {
    let n = mk_proc_stat(ptr::null_mut(), 0);
    (n > 0).then_some(n as u32)
}

/// Calls `f` with every process the kernel listed, the whole table and not a
/// fixed number of rows of it; false when it would not answer. The buffer is
/// sized from the live count, and a table that filled it is read again larger,
/// so a machine running many processes is not judged on the first few.
pub(crate) fn each(mut f: impl FnMut(&ProcStatEntry)) -> bool {
    let Some(live) = live_count() else {
        return false;
    };
    let mut room = live as usize + SLACK;
    for _ in 0..TRIES {
        let mut buf = vec![0u8; HEADER_LEN + room * ENTRY_LEN];
        let written = mk_proc_stat(buf.as_mut_ptr(), room as u32);
        if written <= 0 {
            return false;
        }
        let written = (written as usize).min(room);
        if written == room {
            room *= 2;
            continue;
        }
        for i in 0..written {
            let off = HEADER_LEN + i * ENTRY_LEN;
            /*
             * SAFETY: the bytes at `off` lie inside `buf`, which holds `room`
             * entries after the header, and an entry is plain integers and
             * bytes, valid for any bit pattern.
             */
            let e: ProcStatEntry =
                unsafe { core::ptr::read_unaligned(buf.as_ptr().add(off) as *const ProcStatEntry) };
            f(&e);
        }
        return true;
    }
    false
}

pub(crate) fn name_of(e: &ProcStatEntry) -> &[u8] {
    &e.name[..(e.name_len as usize).min(e.name.len())]
}
