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

/* /proc/<pid>/stat. */

use alloc::vec::Vec;

use super::super::super::view::Proc;
use super::files::{usage, vsize};
use super::statm::segment;

/*
 * Linux's 52 fields, in proc(5)'s order, a row to a group: ids, faults
 * and times, scheduling and memory, code and stack, signals, data and the
 * argument and environment bounds. Zero where Linux has zero too: no
 * terminal, no major faults (nothing is paged in from a disk), no timer,
 * no swap, no delay accounting, one CPU, SCHED_OTHER, no mask of blocked
 * signals, and no exit code while it runs. Two zeros are this view's
 * limits, not Linux's: no kernel flags are kept, and the pending signals
 * are lane C's queue, which the view does not carry yet.
 */
pub(super) fn stat(p: &Proc, tid: u32) -> Vec<u8> {
    let (u, e, r) = (usage(p, tid), &p.exe, &p.reaped);
    let state = if p.sleeping { 'S' } else { 'R' };
    let comm = core::str::from_utf8(&e.comm).unwrap_or("");
    let nice = crate::linux::call::nice_of(p.kernel);
    let (code, data) = (segment(p, true), segment(p, false));
    let start = e.start_ms / (1000 / super::super::super::super::declared::HZ);
    let (threads, rss, vsize) = (p.tids.len(), u.resident_kb / 4, vsize(p));
    let (utime, stime, cutime, cstime) = (u.user, u.system, r.user, r.system);
    let brk = p.brk.0;
    let rows = [
        alloc::format!("{tid} ({comm}) {state} {} {} {} 0 -1 0", p.ppid, p.pgid, p.sid),
        alloc::format!("{} {} 0 0 {utime} {stime} {cutime} {cstime}", u.faults, r.faults),
        alloc::format!("{} {nice} {threads} 0 {start} {vsize} {rss} {}", 20 + nice, u64::MAX),
        alloc::format!("{} {} {} 0 0", code.0, code.1, e.stack),
        alloc::format!("0 0 {} {} 0 0 0 17 0 0 0 0 0 0", p.ignored, p.caught),
        alloc::format!("{} {} {brk} {} {} {} {} 0", data.0, data.1, e.args, e.env, e.env, e.end),
    ];
    (rows.join(" ") + "\n").into_bytes()
}
