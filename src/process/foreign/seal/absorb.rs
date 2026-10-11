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

//! Rows into a live family. Each is nothing for a process no family holds.

use super::super::registry::supervisor_of;
use super::super::trap_table::Answer;
use super::family::Family;
use super::table::{with_family, LIVE};
use super::trace::{Row, REFUSED};

/// The family holding `pid`'s open call: its supervisor's first, so a row
/// never lands in another family, then any, for a guest whose row is gone.
fn with_call(pid: u32, f: impl FnOnce(&mut Family)) {
    let sup = supervisor_of(pid);
    let mut live = LIVE.lock();
    let mine = live.iter().position(|x| Some(x.sup) == sup && x.holds(pid));
    if let Some(at) = mine.or_else(|| live.iter().position(|x| x.holds(pid))) {
        f(&mut live[at]);
    }
}

/// Called with the parked table held, as the answer is posted, so the row
/// and the answer cannot part.
pub(in crate::process::foreign) fn answered(pid: u32, nr: u64, answer: Answer) {
    with_call(pid, |f| f.answered(pid, nr, answer));
}

/// A guest leaving: its unanswered call goes in before its frame goes.
pub(crate) fn abandon(pid: u32) {
    with_call(pid, |f| f.leave(pid));
}

/// A call the kernel turned away itself: a native number, or one past the
/// numbers a supervisor is ever handed. Bucket 512, by its number.
pub(crate) fn refused(nr: u64, ret: u64) {
    let Some(pid) = crate::process::current_pid() else {
        return;
    };
    if let Some(sup) = supervisor_of(pid) {
        with_family(sup, |f| {
            let seq = f.stamp();
            f.absorb(&Row { seq, pid, what: REFUSED, nr, ret });
        });
    }
}

/// A program the calling supervisor had the kernel verify, into its M.
pub(crate) fn measured(measurement: &[u8; 32]) {
    if let Some(caller) = crate::process::current_pid() {
        with_family(caller, |f| f.measured(measurement));
    }
}
