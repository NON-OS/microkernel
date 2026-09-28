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

/* /proc/uptime and /proc/loadavg. */

use alloc::vec::Vec;

use crate::linux::call::family_ms;

use super::super::super::super::declared::HZ;
use super::super::super::super::load;
use super::super::super::synth::num;
use super::super::super::view::View;
use super::files::family;
use super::stat::ticks;

pub(super) fn uptime(v: &View) -> Vec<u8> {
    let (up, idle) = ticks(&family(v));
    let two = |t: u64| alloc::format!("{}.{:02}", t / HZ, t % HZ);
    alloc::format!("{} {}\n", two(up), two(idle)).into_bytes()
}

/* The family's measured load (system/load/), then its running and all threads. */
pub(super) fn loadavg(v: &View) -> Vec<u8> {
    let threads: usize = v.procs.iter().map(|p| p.tids.len()).sum();
    let running = v.procs.iter().filter(|p| !p.sleeping).count();
    let u = family(v);
    let [a, b, c] = load::averages(family_ms(), u.user + u.system).map(load::text);
    let mut s = alloc::format!("{a} {b} {c} {running}/{threads} ").into_bytes();
    s.extend_from_slice(&num(u64::from(last(v))));
    s.push(b'\n');
    s
}

/* The highest number the namespace has given. */
pub(super) fn last(v: &View) -> u32 {
    v.procs.iter().flat_map(|p| p.tids.iter().map(|(ns, _)| *ns)).max().unwrap_or(0)
}
