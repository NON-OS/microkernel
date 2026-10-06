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

/* What each process that has exited used, kept for its parent. */

use alloc::vec::Vec;
use core::cell::RefCell;

use super::usage::Usage;

/*
 * What each process that has exited used, itself and the children it
 * waited for, kept for its parent's RUSAGE_CHILDREN.
 */
pub(super) struct Ended(pub(super) RefCell<Vec<(u32, u32, Usage)>>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Ended {}

static ENDED: Ended = Ended(RefCell::new(Vec::new()));

/* `pid`, child of `parent`, is exiting having used `used`. */
pub fn ended(pid: u32, parent: u32, used: Usage) {
    let mut all = ENDED.0.borrow_mut();
    all.retain(|(p, _, _)| *p != pid);
    all.push((pid, parent, used));
}

/*
 * What `pid`'s children used, those it has waited for, as Linux counts
 * RUSAGE_CHILDREN: `waited` says which.
 */
pub fn children(pid: u32, waited: impl Fn(u32) -> bool) -> Usage {
    let all = ENDED.0.borrow();
    let mut sum = Usage::default();
    for (_, _, u) in all.iter().filter(|(c, p, _)| *p == pid && waited(*c)) {
        sum.user += u.user;
        sum.system += u.system;
        sum.faults += u.faults;
        sum.switches += u.switches;
        sum.resident_kb = sum.resident_kb.max(u.resident_kb);
    }
    sum
}

/* Every thread of the processes in `procs`, by kernel pid. */
pub fn threads_of(procs: &[&crate::linux::file::Proc]) -> Vec<u32> {
    procs.iter().flat_map(|p| p.tids.iter().map(|(_, k)| *k)).collect()
}
