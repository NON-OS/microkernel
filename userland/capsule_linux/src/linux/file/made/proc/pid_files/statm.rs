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

/* /proc/<pid>/statm, and where the program's code and data lie. */

use alloc::vec::Vec;

use super::super::super::view::Proc;
use super::files::usage;

/*
 * Where the program's own code, or its data, starts and ends: its
 * image's executable or writable regions, not the interpreter's. The
 * loader puts a program at its own address below the heap, or a
 * position-independent one at EXEC_BASE, and the interpreter higher.
 */
pub(super) fn segment(p: &Proc, code: bool) -> (u64, u64) {
    use crate::linux::guest::{BRK_BASE, EXEC_BASE, INTERP_BASE};
    let own =
        p.regions.iter().filter(|r| r.at < BRK_BASE || (EXEC_BASE..INTERP_BASE).contains(&r.at));
    let mine: Vec<_> = own.filter(|r| if code { r.exec } else { r.write && !r.exec }).collect();
    let from = mine.iter().map(|r| r.at).min().unwrap_or(0);
    (from, mine.iter().map(|r| r.at + r.len).max().unwrap_or(0))
}

pub(super) fn statm(p: &Proc) -> Vec<u8> {
    let pages = |f: &dyn Fn(&crate::linux::guest::Region) -> bool| {
        p.regions.iter().filter(|r| f(r)).map(|r| r.len / 4096).sum::<u64>()
    };
    let size = pages(&|_| true);
    let text = pages(&|r| r.exec);
    let data = pages(&|r| r.write);
    let rss = usage(p, p.ns).resident_kb / 4;
    alloc::format!("{size} {rss} 0 {text} 0 {data} 0\n").into_bytes()
}
