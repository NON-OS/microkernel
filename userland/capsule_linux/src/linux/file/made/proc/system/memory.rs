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

/* /proc/meminfo and /proc/cpuinfo. */

use alloc::string::String;
use alloc::vec::Vec;

use super::super::super::super::declared::{self as d};
use super::super::super::view::View;
use super::files::family;

/* The x86-64 baseline every x86_64 Linux program may assume, and no more. */
pub(super) fn cpuinfo() -> Vec<u8> {
    let mut s = String::new();
    for n in 0..d::CPUS {
        s += &alloc::format!(
            "processor\t: {n}\nvendor_id\t: NONOS\nmodel name\t: NONOS virtual CPU\n"
        );
        s += "flags\t\t: fpu tsc cx8 cmov mmx fxsr sse sse2 syscall nx lm\n\n";
    }
    s.into_bytes()
}

/*
 * The family's memory: what its processes hold resident, and the copies
 * of files it is writing (held/cache/), which are its page cache and, as a
 * tmpfs's pages are on Linux, its shared memory. Those copies can be put
 * in the store, so they count as available. No swap and no block-device
 * buffers exist.
 */
pub(super) fn meminfo(v: &View) -> Vec<u8> {
    let total = d::MEMORY / 1024;
    let cached = super::super::super::super::cache::bytes() / 1024;
    let free = total.saturating_sub(family(v).resident_kb).saturating_sub(cached);
    let mut s = String::new();
    for (name, kb) in [
        ("MemTotal:", total),
        ("MemFree:", free),
        ("MemAvailable:", free + cached),
        ("Buffers:", 0),
        ("Cached:", cached),
        ("SwapCached:", 0),
        ("SwapTotal:", 0),
        ("SwapFree:", 0),
        ("Shmem:", cached),
    ] {
        s += &alloc::format!("{name:<16}{kb:>8} kB\n");
    }
    s.into_bytes()
}
