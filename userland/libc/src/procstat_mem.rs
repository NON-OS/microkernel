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

//! The machine's memory as the process table's header reports it, read
//! without copying the table: one entry's room is asked for, and the two
//! figures are taken from the header's bytes at their fields' offsets.

use core::mem::{offset_of, size_of};

use crate::procstat::{mk_proc_stat, ProcStatEntry};
use crate::procstat_header::ProcStatHeader;

/// Total and free memory in KiB, or None if the kernel would not say.
pub fn mem_kb() -> Option<(u64, u64)> {
    let mut buf = [0u8; size_of::<ProcStatHeader>() + size_of::<ProcStatEntry>()];
    if mk_proc_stat(buf.as_mut_ptr(), 1) < 0 {
        return None;
    }
    let total = field(&buf, offset_of!(ProcStatHeader, mem_total_kb))?;
    let free = field(&buf, offset_of!(ProcStatHeader, mem_free_kb))?;
    (total > 0 && free <= total).then_some((total, free))
}

fn field(buf: &[u8], at: usize) -> Option<u64> {
    let bytes = buf.get(at..at.checked_add(8)?)?;
    Some(u64::from_le_bytes(bytes.try_into().ok()?))
}
