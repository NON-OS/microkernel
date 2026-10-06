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

use core::mem::size_of;

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

use super::verify::live_count;

const HEADER_LEN: usize = size_of::<ProcStatHeader>();
const ENTRY_LEN: usize = size_of::<ProcStatEntry>();

pub struct Runtime {
    pub capsules: u32,
    pub mem_total_kb: u64,
    pub mem_used_kb: u64,
    pub load: [u64; 3],
}

/// The machine as the kernel counts it now. `capsules` is every live process
/// (`MkProcStat` with no buffer counts them all), and the header comes from a
/// one-row read, since none of its figures depend on the rows. Sizes are
/// kibibytes exactly as the kernel publishes them; `mem_used_kb` is physical
/// memory in use, total less free as the frame allocator counts them, so it
/// covers the kernel and every process. `load` stays raw Q11 (2048 == 1.00) so
/// the formatter owns the rounding.
pub fn sample() -> Option<Runtime> {
    let capsules = live_count()?;
    let mut buf = [0u8; HEADER_LEN + ENTRY_LEN];
    if mk_proc_stat(buf.as_mut_ptr(), 1) <= 0 {
        return None;
    }
    /*
     * SAFETY: `buf` holds a whole header at its start, and a header is plain
     * integers, valid for any bit pattern.
     */
    let header: ProcStatHeader =
        unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const ProcStatHeader) };
    Some(Runtime {
        capsules,
        mem_total_kb: header.mem_total_kb,
        mem_used_kb: header.mem_total_kb.saturating_sub(header.mem_free_kb),
        load: header.load_avg_fixed,
    })
}
