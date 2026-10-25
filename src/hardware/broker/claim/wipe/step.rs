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

//! Moving the wipes on (L78): from a release, an exit, every claim and the
//! x86 tick. Each call takes the records due and not already being stepped,
//! marks them, lets go of the table, does one act on each with no lock held,
//! and puts each back; a record that ended is settled after. No call waits:
//! every window is a deadline the next call checks.

use core::sync::atomic::Ordering;

use super::due_rule::{due, Due};
use super::table::{Wipe, PENDING, WIPES};

const JOBS: usize = 8;

/// `wait`: take the table even if another CPU holds it (a release, an exit,
/// a claim); the tick only tries.
pub(crate) fn step_all(wait: bool) {
    if PENDING.load(Ordering::Acquire) == 0 {
        return;
    }
    let now = crate::time::now_ns();
    let mut jobs: [Option<Wipe>; JOBS] = [None; JOBS];
    {
        let mut all = match wait {
            true => crate::smp::lock_responsive(&WIPES),
            false => match WIPES.try_lock() {
                Some(all) => all,
                None => return,
            },
        };
        let due_now = |w: &&mut Wipe| !w.busy && due(w.phase, now) != Due::Wait;
        for (slot, w) in jobs.iter_mut().zip(all.iter_mut().filter(due_now)) {
            w.busy = true;
            *slot = Some(*w);
        }
    }
    // Each job reads the clock afresh: one before it may have scrubbed and
    // freed a function's kept frames for many milliseconds.
    for job in jobs.into_iter().flatten() {
        let now = crate::time::now_ns();
        let (after, end) = super::act::act(job, now);
        super::finish::put(after, end, crate::time::now_ns());
    }
}
