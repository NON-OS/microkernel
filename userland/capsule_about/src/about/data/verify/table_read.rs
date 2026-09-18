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

//! Filling the tally from the kernel.

use core::mem::size_of;

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

use super::table::{Tally, INIT_NAME, MAX_PROCS};

const HEADER_LEN: usize = size_of::<ProcStatHeader>();
const ENTRY_LEN: usize = size_of::<ProcStatEntry>();

pub fn read(me: u32) -> Option<Tally> {
    let mut buf = [0u8; HEADER_LEN + MAX_PROCS * ENTRY_LEN];
    let written = mk_proc_stat(buf.as_mut_ptr(), MAX_PROCS as u32);
    if written <= 0 {
        return None;
    }
    let count = (written as usize).min(MAX_PROCS);
    let mut t = Tally {
        masks: [0; MAX_PROCS],
        is_init: [false; MAX_PROCS],
        total: 0,
        unmasked: 0,
        own_mask: 0,
    };
    for i in 0..count {
        let off = HEADER_LEN + i * ENTRY_LEN;
        if off + ENTRY_LEN > buf.len() {
            break;
        }
        let e: ProcStatEntry =
            unsafe { core::ptr::read_unaligned(buf.as_ptr().add(off) as *const ProcStatEntry) };
        let slot = t.total as usize;
        t.masks[slot] = e.caps;
        t.is_init[slot] = name_of(&e) == INIT_NAME;
        t.total += 1;
        if e.caps == 0 {
            t.unmasked += 1;
        }
        // 0 is the kernel's "unknown" answer for a pid, and no process has it.
        if me != 0 && e.pid == me {
            t.own_mask = e.caps;
        }
    }
    Some(t)
}

fn name_of(e: &ProcStatEntry) -> &[u8] {
    &e.name[..(e.name_len as usize).min(e.name.len())]
}
