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

//! Filling it from the kernel.

use core::mem::size_of;

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

use super::types::{Capsule, Snapshot, MAX_PROCS};

const HEADER_LEN: usize = size_of::<ProcStatHeader>();
const ENTRY_LEN: usize = size_of::<ProcStatEntry>();

/// Read the table. `None` when the kernel will not answer, which every caller
/// must forward as "not established" rather than as an empty machine.
pub fn snapshot() -> Option<Snapshot> {
    let mut buf = [0u8; HEADER_LEN + MAX_PROCS * ENTRY_LEN];
    let written = mk_proc_stat(buf.as_mut_ptr(), MAX_PROCS as u32);
    if written <= 0 {
        return None;
    }
    let count = (written as usize).min(MAX_PROCS);
    let mut snap = Snapshot {
        capsules: [const { Capsule { name: [0; 24], name_len: 0, caps: 0 } }; MAX_PROCS],
        count: 0,
    };
    for i in 0..count {
        let off = HEADER_LEN + i * ENTRY_LEN;
        if off + ENTRY_LEN > buf.len() {
            break;
        }
        let e: ProcStatEntry =
            unsafe { core::ptr::read_unaligned(buf.as_ptr().add(off) as *const ProcStatEntry) };
        let n = (e.name_len as usize).min(24);
        let slot = &mut snap.capsules[snap.count];
        slot.name[..n].copy_from_slice(&e.name[..n]);
        slot.name_len = n as u8;
        slot.caps = e.caps;
        snap.count += 1;
    }
    Some(snap)
}
