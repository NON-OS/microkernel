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

//! step.rs `step_all`, a stage a move: the clock read (none pending: back at
//! once), the table taken (the tick only tries) and the due records no step
//! is acting on marked busy, the table let go, the act with no lock held,
//! then finish.rs: an ended wipe's kept frames settled outside every lock
//! (model_end), and only then its record put back under the table (L80).

use super::model_act::act;
use super::model_end::{after_put, before_put, settle};
use super::model_finish::{collect, put};
use super::model_stepper::Stepper;
use super::phase_rule::MS;
use super::world::World;
use super::world_cases::CONTEND;
use super::world_knob::Knob;

/// One stage; the delay before the next, or none once the call returned.
pub(super) fn stage(w: &mut World, s: &mut Stepper, me: usize) -> Option<u64> {
    let now = if w.knob == Knob::StallFresh { w.now } else { s.now };
    let stalls = matches!(w.knob, Knob::Stall | Knob::StallFresh);
    s.stage += 1;
    match s.stage - 1 {
        0 if w.pending == 0 => return None,
        0 => s.now = w.now,
        1 if w.wipes_by.is_some() => {
            w.seen[CONTEND] += 1;
            return None;
        }
        1 => collect(w, s, me),
        2 => {
            w.wipes_by = None;
            s.job?;
            if stalls && w.chance(3) {
                return Some(MS * w.draw(41));
            }
        }
        3 => (s.job, s.end) = act(w, s.job?, now),
        4 => return Some(before_put(w, s)),
        5 => put(w, s, me, now),
        6 => {
            w.wipes_by = None;
            return after_put(w, s);
        }
        _ => {
            settle(w, s);
            return None;
        }
    }
    Some(0)
}
