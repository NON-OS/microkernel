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

/* Locks whose owner is gone, and the namespace's numbers of the owners. */

use super::super::super::view;
use super::super::lock::{self, Owner};

/*
 * Drop the locks whose owners are gone from the family, when the family's
 * view is lent: a POSIX lock whose process has exited, and a flock or OFD
 * lock whose description no process holds any more.
 */
pub fn purge() {
    view::with(|v| {
        if v.procs.is_empty() {
            return;
        }
        let holds = |d: u32| v.procs.iter().any(|p| p.fds.iter().any(|o| o.desc == Some(d)));
        lock::drop_where(|l| match l.owner {
            Owner::Posix(pid) => !v.procs.iter().any(|p| p.kernel == pid),
            Owner::Flock(d) | Owner::Ofd(d) => !holds(d),
        });
    });
}

pub(crate) fn owners_ns(owner: Owner) -> i32 {
    match owner {
        Owner::Posix(k) => {
            view::with(|v| v.procs.iter().find(|p| p.kernel == k).map_or(0, |p| p.ns as i32))
        }
        _ => -1,
    }
}
