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

/* The kernel's counts for the threads asked about, from its table. */

use alloc::vec;
use core::mem::size_of;
use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

/*
 * Sums over the threads asked about. Ticks are the kernel's 100 Hz ticks,
 * which is also the guest's clock tick (AT_CLKTCK, declared::HZ).
 */
#[derive(Clone, Copy, Default)]
pub struct Usage {
    pub user: u64,
    pub system: u64,
    pub faults: u64,
    pub switches: u64,
    pub resident_kb: u64,
}

const HEADER: usize = size_of::<ProcStatHeader>();

const ENTRY: usize = size_of::<ProcStatEntry>();

pub fn usage(pids: &[u32]) -> Usage {
    let mut sum = Usage::default();
    let count = mk_proc_stat(core::ptr::null_mut(), 0);
    if count <= 0 || pids.is_empty() {
        return sum;
    }
    /* Room for a few more, in case the machine starts one between the calls. */
    let room = count as usize + 8;
    let mut buf = vec![0u8; HEADER + room * ENTRY];
    let written = mk_proc_stat(buf.as_mut_ptr(), room as u32);
    for i in 0..written.max(0) as usize {
        let at = HEADER + i * ENTRY;
        let Some(raw) = buf.get(at..at + ENTRY) else { break };
        /*
         * SAFETY: the slice holds ENTRY bytes, and every bit pattern is a
         * valid ProcStatEntry, which is plain integers and bytes.
         */
        let e: ProcStatEntry = unsafe { core::ptr::read_unaligned(raw.as_ptr().cast()) };
        if pids.contains(&e.pid) {
            sum.user += e.user_ticks;
            sum.system += e.run_ticks.saturating_sub(e.user_ticks);
            sum.faults += e.faults;
            sum.switches += e.switches;
            sum.resident_kb += e.mem_kb;
        }
    }
    sum
}
