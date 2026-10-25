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

//! act.rs: one act on a wipe, no lock held. Start reads the power state,
//! Transactions Pending and Command, saves what a reset clears, sets the
//! marker and reads it back, and asks `start` whether to write the reset
//! now; ToD0 (model_reset) ends a power cycle's trip; a check is
//! model_check's.

use super::bits_rule::{BME, PM_STATE};
use super::due_rule::Due;
use super::end_rule::End;
use super::method_rule::method;
use super::model_check::check;
use super::model_reset::{mark, to_d0, write};
use super::model_stepper::due_at;
use super::phase_rule::Phase;
use super::start_rule::{start as rule, Do, Look};
use super::world::World;
use super::world_cases::TP_STAY;
use super::world_knob::Knob;
use super::world_types::{Saved, Wipe};

pub(super) fn act(w: &mut World, job: Wipe, now: u64) -> (Option<Wipe>, Option<End>) {
    let (job, end) = match due_at(w.knob, job.phase, now) {
        Due::Start => start(w, job, now),
        Due::ToD0 => (to_d0(w, job, now), None),
        Due::Check => check(w, job, now),
        Due::Wait => (job, None),
    };
    (Some(job), end)
}

/// act.rs `start`; undo check 14 reads Transactions Pending as clear.
fn start(w: &mut World, mut job: Wipe, now: u64) -> (Wipe, Option<End>) {
    let Phase::Owed { since } = job.phase else { return (job, None) };
    let c = w.dev.caps;
    let in_d0 = !c.pm || w.pmcsr() & PM_STATE == 0;
    let tp = (c.pcie || c.af_tp) && w.tp() && w.knob != Knob::NoTpWait;
    let command = w.command();
    job.bme_off = command & BME == 0;
    let m = job.method.or(method(c.offers));
    if m.is_some() && job.saved.is_none() {
        // reset.rs `save`: Device Control, then DevCtl2, CLS, LT, IntLine.
        let devctl = c.pcie.then(|| w.devctl());
        (0..4).for_each(|_| w.touch(false));
        job.saved = Some(Saved { command, devctl });
    }
    if m.is_some() && job.saved.is_some() && job.marker.is_none() {
        job.marker = mark(w);
    }
    match rule(m, Look { in_d0, tp, marker: job.marker }, since, now) {
        Do::Stay => {
            w.seen[TP_STAY] += 1;
            (job, None)
        }
        Do::Unreset => (job, Some(End::Unreset)),
        Do::Write(m, _) => (write(w, job, m, now), None),
    }
}
