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

//! A stepper's one act on the device in the enter model (L78), with no lock
//! held, as act.rs, reset.rs and check.rs do it: the reset started or found
//! not possible, the move back to D0, or what a written reset came to.

use super::due_rule::{due, Due};
use super::end_rule::{checked, End, Next};
use super::enter_parts::Job;
use super::enter_world::World;
use super::marker_rule::Marker;
use super::method_rule::{fallback, method, Method, Offers};
use super::phase_rule::Phase;
use super::start_rule::{start as rule, written, Do, Look};

pub(super) fn act(w: &mut World) -> bool {
    let Some(mut j) = w.job else { return false };
    let o = Offers { flr: w.fate.flr, ..Offers::default() };
    match (due(j.rec.phase, j.now), j.rec.phase) {
        (Due::Start, Phase::Owed { since }) => start(w, &mut j, o, since),
        (Due::ToD0, _) => j.rec.phase = Phase::D0 { at: j.now },
        // check::check: answering as itself, marker clear, by the clock; the
        // rule is asked at the pass's time, step_all's one reading for all.
        (Due::Check, p) => {
            let ready = w.written.is_some_and(|t| w.now >= t.saturating_add(w.fate.back));
            let fall = j.rec.method.and_then(|m| fallback(m, o));
            match checked(p, j.now, ready, ready && j.rec.marker && !w.marker, fall) {
                Next::Stay => {}
                Next::Retry(m) => write(w, &mut j, m),
                Next::Ended(e) => j.end = Some(e),
            }
        }
        _ => {}
    }
    w.job = Some(j);
    true
}

// act::start: with a method, the marker set and read back first.
fn start(w: &mut World, j: &mut Job, o: Offers, since: u64) {
    let m = j.rec.method.or(method(o));
    w.marker |= m.is_some() && !j.rec.marker;
    j.rec.marker |= m.is_some();
    let look = Look { in_d0: true, tp: false, marker: j.rec.marker.then_some(Marker::IntxDisable) };
    match rule(m, look, since, j.now) {
        Do::Stay => {}
        Do::Unreset => j.end = Some(End::Unreset),
        Do::Write(m, _) => write(w, j, m),
    }
}

// reset::write, at the clock's time, the phase stamped with the pass's: a
// reset that reaches the registers clears them and the marker, but a display
// function's memory may outlive it (recalled, unverified).
fn write(w: &mut World, j: &mut Job, m: Method) {
    if w.fate.obeys {
        (w.marker, w.dirty) = (false, w.dirty.filter(|_| w.fate.display));
    }
    w.written = Some(w.now);
    (j.rec.method, j.rec.phase) = (Some(m), written(m, j.now));
}
