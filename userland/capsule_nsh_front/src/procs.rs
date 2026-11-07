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

//! The process table for `ps`, from MkProcStat. nsh.front holds neither
//! AttestRead nor ProcessControl, so the kernel hands it every process's
//! pid, parent, state and name with the counters cleared; that is all `ps`
//! prints.

use alloc::vec;
use alloc::vec::Vec;
use core::mem::{offset_of, size_of};

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader, PROC_NAME_LEN};
use nonos_shell::host::Proc;

/// The most rows one `ps` reads; a table longer than this is cut, as the
/// terminal's is.
const MAX_ROWS: usize = 512;

pub fn table() -> Option<Vec<Proc>> {
    let count = usize::try_from(mk_proc_stat(core::ptr::null_mut(), 0)).ok()?;
    let max = count.checked_add(8)?.min(MAX_ROWS);
    let row = size_of::<ProcStatEntry>();
    let mut buf = vec![0u8; size_of::<ProcStatHeader>().checked_add(max.checked_mul(row)?)?];
    let got = usize::try_from(mk_proc_stat(buf.as_mut_ptr(), u32::try_from(max).ok()?)).ok()?;
    let rows = buf.get(size_of::<ProcStatHeader>()..)?;
    Some(rows.chunks_exact(row).take(got.min(max)).filter_map(proc).collect())
}

fn proc(e: &[u8]) -> Option<Proc> {
    let len = usize::from(*e.get(offset_of!(ProcStatEntry, name_len))?).min(PROC_NAME_LEN);
    let at = offset_of!(ProcStatEntry, name);
    Some(Proc {
        pid: word(e, offset_of!(ProcStatEntry, pid))?,
        ppid: word(e, offset_of!(ProcStatEntry, ppid))?,
        state: *e.get(offset_of!(ProcStatEntry, state))?,
        name: e.get(at..at.checked_add(len)?)?.to_vec(),
    })
}

fn word(e: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(e.get(at..at.checked_add(4)?)?.try_into().ok()?))
}
