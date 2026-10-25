// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The leave phase's schedule, before power goes: every disk driver asked
//! at once, each waited on for no more than its own budget, the whole for no
//! more than the total, so a machine with four disks powers off within the
//! total and never waits on one disk for ever. Pure: the legs, the clock and
//! the rest between rounds are passed in, so kernel_proofs holds the bounds.

use alloc::vec::Vec;

pub(crate) use super::poll::{Ended, Poll};

/// The most the leave phase takes, however many disks there are.
pub(crate) const TOTAL_MS: u64 = 15_000;
/// The most it takes when a thermal zone or the battery is critical and the
/// machine must go now: enough for an NVMe shutdown and a disk's flush on a
/// working machine, short of what a failing one may want.
pub(crate) const CRITICAL_TOTAL_MS: u64 = 6_000;

/// Run every leg until each has ended. `budgets[i]` bounds leg `i`, measured
/// from the first clock read; `total_ms` bounds them all. `poll(i)` is asked
/// once a round of each leg still running; `rest` runs between rounds.
pub(crate) fn run<C, P, R>(
    budgets: &[u64],
    total_ms: u64,
    mut now: C,
    mut poll: P,
    mut rest: R,
) -> Vec<Ended>
where
    C: FnMut() -> Option<u64>,
    P: FnMut(usize) -> Poll,
    R: FnMut(),
{
    let mut ended: Vec<Option<Ended>> = budgets.iter().map(|_| None).collect();
    let Some(start) = now() else { return budgets.iter().map(|_| Ended::NoClock).collect() };
    loop {
        let Some(t) = now() else {
            return ended.into_iter().map(|e| e.unwrap_or(Ended::NoClock)).collect();
        };
        let spent = t.saturating_sub(start);
        for (i, slot) in ended.iter_mut().enumerate() {
            if slot.is_some() {
                continue;
            }
            *slot = if spent >= budgets[i].min(total_ms) {
                Some(Ended::Missed)
            } else {
                match poll(i) {
                    Poll::Waiting => None,
                    Poll::Answered(status) => Some(Ended::Answered(status)),
                    Poll::Gone => Some(Ended::Gone),
                }
            };
        }
        if ended.iter().all(Option::is_some) {
            return ended.into_iter().flatten().collect();
        }
        rest();
    }
}
