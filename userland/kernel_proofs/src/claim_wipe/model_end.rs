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

//! finish.rs before the record is written: an Unreset end's marker cleared,
//! then the ended wipe's kept frames settled outside every lock (L80), a
//! lost one's left for its next end (undo check 8 frees them past Bus Master
//! too). Undo checks 4 and 17 settle as before L80, once the record is let
//! go, so a claim let in over the end meets frames not yet counted as kept
//! (I10), or, with no epoch to key the settle, runs it may free (I2).

use super::end_rule::{frees, End};
use super::model_check::unmark;
use super::model_stepper::Stepper;
use super::phase_rule::MS;
use super::world::World;
use super::world_knob::Knob;

const LATE: [Knob; 2] = [Knob::SettleNoEpoch, Knob::SettleLate];

/// The stage before the record is written; the scrub's time after it.
pub(super) fn before_put(w: &mut World, s: &mut Stepper) -> u64 {
    if let (Some(job), Some(End::Unreset)) = (s.job, s.end) {
        unmark(w, &job);
    }
    if LATE.contains(&w.knob) || !settle(w, s) {
        return 0;
    }
    scrub(w)
}

/// The stage once the record is let go: the scrub's time before a late
/// settle, else none, the call over.
pub(super) fn after_put(w: &mut World, s: &Stepper) -> Option<u64> {
    (LATE.contains(&w.knob) && keeps(w, s).is_some()).then(|| scrub(w))
}

/// kept_settle.rs `settle_kept`, keyed by the job's epoch; false if there
/// is no end to settle.
pub(super) fn settle(w: &mut World, s: &mut Stepper) -> bool {
    let (Some(job), Some(keep)) = (s.job, keeps(w, s)) else { return false };
    w.settle(job.epoch, !keep);
    s.settled = true;
    true
}

/// Whether the job's end, still to be settled, keeps its frames for good.
fn keeps(w: &World, s: &Stepper) -> Option<bool> {
    let (job, end) = (s.job?, s.end.filter(|&e| e != End::Lost && !s.settled)?);
    Some(!frees(end, job.bme_off, w.dev.caps.binds && w.knob != Knob::FreeWhenBinds))
}

/// The bytes ended wipes keep for good but have not settled yet (I10).
pub(super) fn unsettled(w: &World) -> u64 {
    let keep = |s: Stepper| s.job.map(|j| j.epoch).filter(|_| keeps(w, &s) == Some(true));
    let ending: Vec<_> = w.threads.iter().filter_map(|t| t.stepper.and_then(keep)).collect();
    w.kept.iter().filter(|k| !k.3 && ending.contains(&k.1)).map(|k| w.runs[k.0].len).sum()
}

fn scrub(w: &mut World) -> u64 {
    MS * if w.chance(4) { w.draw(301) } else { 0 }
}
