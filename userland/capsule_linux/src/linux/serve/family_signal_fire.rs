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

//! The timers of a process that are due: ITIMER_REAL raises SIGALRM, a POSIX
//! timer raises its own signal or counts an overrun while that still waits,
//! and a sigtimedwait whose time is up answers EAGAIN.

use super::family_wait::answer;
use crate::linux::abi::errno;
use crate::linux::guest::siginfo::{SigInfo, SI_KERNEL, SI_TIMER};
use crate::linux::guest::sigstate::SIGALRM;
use crate::linux::guest::Guest;

/// Every timer of `g` due by `now`, fired, and every sigtimedwait whose time
/// is up answered EAGAIN.
pub fn fire(g: &mut Guest, now: u64) {
    if let Some(mut t) = g.signals.real.filter(|t| t.due <= now) {
        let _ = g.signals.raise(0, SigInfo::from(SIGALRM, SI_KERNEL, 0));
        g.signals.real = t.rearm(now).then_some(t);
    }
    for i in 0..g.signals.timers.len() {
        let mut t = g.signals.timers[i];
        if t.due.is_none_or(|d| d > now) {
            continue;
        }
        if t.queued {
            t.overrun = t.overrun.saturating_add(1);
        } else if t.signo != 0 {
            let info = SigInfo {
                timer: Some((t.id, 0)),
                value: t.value,
                ..SigInfo::from(t.signo, SI_TIMER, 0)
            };
            t.queued = g.signals.raise(t.tid, info);
        }
        let _ = t.rearm(now);
        g.signals.timers[i] = t;
    }
    let late: alloc::vec::Vec<u32> = g
        .signals
        .sigwaits
        .iter()
        .filter(|w| w.due.is_some_and(|d| d <= now))
        .map(|w| w.tid)
        .collect();
    for tid in late {
        g.signals.sigwaits.retain(|w| w.tid != tid);
        answer(g, tid, errno::fail(errno::EAGAIN));
    }
}
