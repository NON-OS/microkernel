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

//! pending.rs `admit`: the claim made Ready and its record written Held in
//! one claims hold, then I1 judged on who was let in, over what: never over
//! another service's state still in the function (a confirmed reset of a
//! function that is not a display clears it), never over another service's
//! kept function, never within a lost function's hold.

use super::end_rule::End;
use super::holder_rule::same;
use super::model_stage::types::Stage;
use super::phase_rule::Phase;
use super::world::World;
use super::world_cases::{A2_KEPT, B_CLEAN, HOLD_LOST, LOST_ADMIT};
use super::world_thread::Thread;
use super::world_types::Entry;

pub(super) fn admit(w: &mut World, p: usize, t: &mut Thread) {
    let making = |e: Entry| (e.pid, e.epoch, e.stage) == (p, t.epoch, Stage::Making);
    if !w.entry.is_some_and(making) {
        t.step = 9;
    } else if !w.held(t.epoch, w.procs[p].name) {
        w.bad(5, "admit met a wipe under way");
        t.step = 9;
    } else {
        w.entry = w.entry.map(|e| Entry { stage: Stage::Ready, ..e });
        admitted(w, p);
        w.start_tenancy(p, t);
    }
}

fn admitted(w: &mut World, p: usize) {
    let (svc, before) = (w.procs[p].svc, w.procs[p].before);
    if w.dev.secret.is_some_and(|s| s != svc) {
        w.bad(1, "let in over another service's state in the function");
    }
    let kept = before.is_some_and(|r| r.phase == Phase::Kept);
    if kept && !same(before.and_then(|r| r.by), w.procs[p].name) {
        w.bad(1, "let in over another service's kept function");
    }
    if w.truth.lost_at.is_some_and(|at| w.now < at.saturating_add(HOLD_LOST)) {
        w.bad(1, "let in within a lost function's hold");
    }
    let clean = before.is_none() && w.truth.last_end == Some(End::Clean);
    w.seen[B_CLEAN] += (p == 2 && clean) as u32;
    w.seen[A2_KEPT] += (p == 1 && kept) as u32;
    w.seen[LOST_ADMIT] += w.truth.rearmed as u32;
    (w.truth.lost_at, w.truth.rearmed) = (None, false);
}
