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

//! A claimant's side, libc's `mk_device_claim`: Resetting (EAGAIN) asked
//! again for 5 s from the first try or up to its count of tries, each sleep
//! ended early by any wake (1 to 10 ms). One begun on a lost function back
//! past its hold never ends with that loss standing (I8).

use super::model_stage::types::ClaimError;
use super::phase_rule::{Phase, MS};
use super::world::World;
use super::world_cases::{BOUND, CAP_LOST, HOLD_LOST, RETRIED};
use super::world_knob::Knob;
use super::world_thread::{Path, Thread};

/// libc's bound, past the longest a wipe stays pending (L80), and its count
/// of tries, which sleeps of 1 ms or more never spend first (both libc's).
pub(super) const RESET_MS: u64 = 5000 * MS;
pub(super) const TRY_CAP: u32 = 5000;
const _: () = assert!(RESET_MS > BOUND && TRY_CAP as u64 * MS >= RESET_MS);

pub(super) fn begin(w: &mut World, p: usize, t: &mut Thread) {
    let (now, ready) = (w.now, w.dev.ready_at);
    let lost = w.rec.and_then(|r| match r.phase {
        Phase::Lost { at } if now >= at.saturating_add(HOLD_LOST) && now >= ready => Some(at),
        _ => None,
    });
    let pr = &mut w.procs[p];
    (pr.owed_back, pr.started, pr.tries) = (lost, now, 0);
    (t.path, t.step) = (if pr.dead { Path::Done } else { Path::Claim }, 0);
}

pub(super) fn refused(w: &mut World, p: usize, t: &mut Thread, e: ClaimError) {
    let resetting = e == ClaimError::Resetting && !w.procs[p].dead;
    let cap = if w.knob == Knob::FewTries { 250 } else { TRY_CAP };
    let pr = &mut w.procs[p];
    if resetting && w.now < pr.started + RESET_MS && pr.tries < cap {
        pr.tries += 1;
        w.seen[RETRIED] += 1;
        (t.step, t.at) = (0, w.now + MS * (1 + w.draw(10)));
        return;
    }
    if resetting {
        w.bad(9, "libc gave up on a function still being reset (I9)");
    }
    attempt_over(w, p, t);
}

pub(super) fn attempt_over(w: &mut World, p: usize, t: &mut Thread) {
    let pr = w.procs[p];
    let still = pr.owed_back.is_some_and(|at| w.rec.is_some_and(|r| r.phase == Phase::Lost { at }));
    if still && !pr.dead {
        w.bad(8, "a function back past its hold left lost");
    }
    w.seen[CAP_LOST] += (still && !pr.dead && pr.over_cap) as u32;
    let pr = &mut w.procs[p];
    pr.attempts = pr.attempts.saturating_sub(1);
    t.path = if pr.attempts == 0 || pr.dead { Path::Done } else { Path::Idle };
    if t.path == Path::Idle {
        t.at = w.now + MS * w.draw(2501);
    }
}
