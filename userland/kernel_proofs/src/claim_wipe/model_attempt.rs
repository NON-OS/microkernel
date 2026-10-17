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

//! A claimant's side, libc's `mk_device_claim`: an attempt begun in user
//! code, Resetting (EAGAIN) asked again 10 ms on up to 250 times, any other
//! answer the attempt's end. An attempt begun on a lost function past its
//! hold, and back by then, must not end with that loss still standing (I8),
//! unless the kept frames' bound refused it: as built that is asked first.

use super::model_stage::types::ClaimError;
use super::phase_rule::{Phase, MS};
use super::world::World;
use super::world_cases::{CAP_LOST, HOLD_LOST, RETRIED};
use super::world_thread::{Path, Thread};

pub(super) fn begin(w: &mut World, p: usize, t: &mut Thread) {
    if w.procs[p].dead {
        t.path = Path::Done;
        return;
    }
    let now = w.now;
    let lost = w.rec.and_then(|r| match r.phase {
        Phase::Lost { at } if now >= at.saturating_add(HOLD_LOST) => Some(at),
        _ => None,
    });
    w.procs[p].owed_back = lost.filter(|_| now >= w.dev.ready_at);
    w.procs[p].tries = 0;
    (t.path, t.step) = (Path::Claim, 0);
}

pub(super) fn refused(w: &mut World, p: usize, t: &mut Thread, e: ClaimError) {
    let pr = &mut w.procs[p];
    if e == ClaimError::Resetting && pr.tries < 250 && !pr.dead {
        pr.tries += 1;
        w.seen[RETRIED] += 1;
        (t.step, t.at) = (0, w.now + 10 * MS);
        return;
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
    if pr.attempts == 0 || pr.dead {
        t.path = Path::Done;
        return;
    }
    t.path = Path::Idle;
    t.at = w.now + MS * w.draw(2501);
}
