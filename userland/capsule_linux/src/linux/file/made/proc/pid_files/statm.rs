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
