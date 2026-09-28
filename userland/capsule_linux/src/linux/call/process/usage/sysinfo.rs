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

/* sysinfo: the family's uptime, load, memory and threads. */

use crate::linux::abi::errno;
use crate::linux::file::{self, declared};
use crate::linux::guest::Guest;

use super::super::super::family_ms;

pub fn sysinfo(guest: &Guest, out: u64) -> u64 {
    let (threads, resident, ran) = file::view_with(|v| {
        let leaders: alloc::vec::Vec<u32> = v.procs.iter().map(|p| p.kernel).collect();
        let all: alloc::vec::Vec<&file::Proc> = v.procs.iter().collect();
        let u = file::cpu::usage(&file::cpu::threads_of(&all));
        let n: usize = v.procs.iter().map(|p| p.tids.len()).sum();
        (n.max(1), file::cpu::usage(&leaders).resident_kb * 1024, u.user + u.system)
    });
    let loads = file::load::averages(family_ms(), ran);
    let mut b = [0u8; 112];
    let mut put = |at: usize, v: u64| b[at..at + 8].copy_from_slice(&v.to_le_bytes());
    put(0, family_ms() / 1000); /* uptime */
    for (i, avg) in loads.iter().enumerate() {
        put(8 + i * 8, avg << 5); /* loads, from Linux's 11 bits to sysinfo's 16 */
    }
    put(32, declared::MEMORY); /* totalram */
    put(40, declared::MEMORY.saturating_sub(resident)); /* freeram */
    b[80..82].copy_from_slice(&(threads.min(u16::MAX as usize) as u16).to_le_bytes()); /* procs */
    b[104..108].copy_from_slice(&1u32.to_le_bytes()); /* mem_unit */
    match guest.write(out, &b) {
        112 => errno::ok(0),
        _ => errno::fail(errno::EFAULT),
    }
}
