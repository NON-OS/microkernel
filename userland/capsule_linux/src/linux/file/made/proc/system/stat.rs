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

/* /proc/stat, and the ticks the family ran and did not. */

use alloc::vec::Vec;

use crate::linux::call::family_ms;

use super::super::super::super::cpu::Usage;
use super::super::super::super::declared::{self as d, HZ};
use super::super::super::view::View;
use super::files::family;
use super::time::last;

/* Ticks since the family started, and the part no thread of it ran. */
pub(super) fn ticks(u: &Usage) -> (u64, u64) {
    let up = family_ms() / (1000 / HZ);
    (up, up.saturating_sub(u.user + u.system))
}

pub(super) fn stat(v: &View) -> Vec<u8> {
    let u = family(v);
    let (_, idle) = ticks(&u);
    let cpu = alloc::format!("{} 0 {} {idle} 0 0 0 0 0 0", u.user, u.system);
    let wall = u64::try_from(nonos_libc::mk_time_millis()).unwrap_or(0);
    let btime = wall.saturating_sub(family_ms()) / 1000;
    let running = v.procs.iter().filter(|p| !p.sleeping).count();
    /* The one CPU is the whole machine, so it and the total are the same line. */
    let mut s = alloc::format!("cpu  {cpu}\n");
    for n in 0..d::CPUS {
        s += &alloc::format!("cpu{n} {cpu}\n");
    }
    s += &alloc::format!("intr 0\nctxt {}\nbtime {btime}\n", u.switches);
    s += &alloc::format!(
        "processes {}\nprocs_running {running}\nprocs_blocked 0\n",
        last(v).saturating_sub(1)
    );
    s += "softirq 0 0 0 0 0 0 0 0 0 0 0\n";
    s.into_bytes()
}
