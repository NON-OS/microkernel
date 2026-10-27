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

//! What holds after every call of the model: each map shows its file as a
//! Linux page cache holds it, and a map that cannot write reads zero past
//! the end; the table keeps the one writer rule and the bound.

use super::model::{ix, World};
use crate::linux::file::shared_maps::rules::{conflict, MAX_MAPS};

pub fn check(w: &World) {
    for e in w.table.iter().filter(|e| !e.orphan) {
        let (truth, (lo, hi)) = (&w.truth[ix(&e.path)], e.file());
        let end = (truth.len() as u64).clamp(lo, hi);
        let seen = &w.mem[e.pid as usize][e.addr(lo) as usize..][..e.span as usize];
        let want = truth.get(lo as usize..end as usize).unwrap_or_default();
        assert_eq!(&seen[..(end - lo) as usize], want, "{e:?}");
        let past = &seen[(end - lo) as usize..];
        assert!(e.writer || past.iter().all(|b| *b == 0), "past the end: {e:?}");
    }
    let t = &w.table;
    assert!(t.len() <= MAX_MAPS);
    assert!(t.iter().enumerate().all(|(i, a)| t[i + 1..].iter().all(|b| !conflict(a, b))));
}
