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

pub(super) fn stat(p: &Proc, tid: u32) -> Vec<u8> {
    let u = usage(p, tid);
    let state = if p.sleeping { 'S' } else { 'R' };
    let comm = core::str::from_utf8(&p.exe.comm).unwrap_or("");
    let start = p.exe.start_ms / (1000 / super::super::super::super::declared::HZ);
    let (e, rss) = (&p.exe, u.resident_kb / 4);
    let head = alloc::format!(
        "{tid} ({comm}) {state} {} {} {} 0 -1 0 {} 0 0 0 {} {} 0 0 {} {} {} 0 {start} {} {rss} \
         18446744073709551615 0 0 0 0 0 0 0 {} {} 0 0 0 17 0 0 0 0 0 0 0 0 {} {} {} {} {} 0\n",
        p.ppid,
        p.pgid,
        p.sid,
        u.faults,
        u.user,
        u.system,
        20,
        0,
        p.tids.len(),
        vsize(p),
        p.ignored,
        p.caught,
        p.brk.0,
        e.args,
        e.env,
        e.env,
        e.end,
    );
    head.into_bytes()
}
