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

/*
 * getrusage: what a process, a thread or the children it waited for
 * used, in the kernel's ticks for the family's own threads.
 */

use crate::linux::abi::errno;
use crate::linux::file::{self};
use crate::linux::guest::Guest;

use super::times::{children, mine, TICK_MS};

const RUSAGE_SELF: i64 = 0;

const RUSAGE_CHILDREN: i64 = -1;

const RUSAGE_THREAD: i64 = 1;

pub fn getrusage(guest: &Guest, tid: u32, who: u64, out: u64) -> u64 {
    let used = match who as i64 {
        RUSAGE_SELF => mine(guest),
        RUSAGE_THREAD => file::cpu::usage(&[tid]),
        RUSAGE_CHILDREN => children(guest),
        _ => return errno::fail(errno::EINVAL),
    };
    let mut b = [0u8; 144];
    let mut put = |at: usize, v: u64| b[at..at + 8].copy_from_slice(&v.to_le_bytes());
    let tv = |ticks: u64| ((ticks * TICK_MS) / 1000, (ticks * TICK_MS) % 1000 * 1000);
    let (us, uu) = tv(used.user);
    let (ss, su) = tv(used.system);
    put(0, us);
    put(8, uu);
    put(16, ss);
    put(24, su);
    put(32, used.resident_kb); /* ru_maxrss: the resident size now, no peak being kept */
    put(64, used.faults); /* ru_minflt */
    put(128, used.switches); /* ru_nvcsw: every switch the kernel counted */
    match guest.write(out, &b) {
        144 => errno::ok(0),
        _ => errno::fail(errno::EFAULT),
    }
}
