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

//! The rules, each a function of a state or of a step taken from one.

use super::actors::{Hold, Taker, World};
use super::drawer_steps::probe;
use super::events::Event;
use super::gate::Gate;
use super::outcome::Broken;

/// The rule a step breaks: a wait that finds quiet while a draw runs.
pub fn step(before: &World, e: Event) -> Option<Broken> {
    let drawing = before.drawers.iter().any(|d| d.drawing());
    (e == Event::Quiet && drawing).then_some(Broken::QuietMidDraw)
}

/// Every rule a state breaks.
pub fn state(g: &Gate, w: &World) -> Vec<Broken> {
    let drawing = w.drawers.iter().any(|d| d.drawing());
    let quiet = |t: &Taker| matches!(t, Taker::Quiet | Taker::Latched | Taker::Freed);
    let believes = w.takers.iter().any(quiet) || w.hold == Hold::Quiet;
    let made = |t: &Taker| matches!(t, Taker::Latched | Taker::Ended { ready: true });
    let ready = w.takers.iter().any(made);
    let idle = |t: &Taker| matches!(t, Taker::Off | Taker::Start | Taker::Ended { ready: false });
    let free = matches!(w.hold, Hold::Off | Hold::Start | Hold::Waiting | Hold::Cleared);
    let clear = w.takers.iter().all(idle) && free;
    let f = &w.flags;
    let alone = g.reads.iter().any(|&r| f.stop(r));
    let draws = probe(g, f);
    [
        (believes && drawing, Broken::DrawWhileQuiet),
        (f.gone && drawing, Broken::DrawAfterGone),
        (ready && drawing, Broken::ReadyMidDraw),
        (clear && !draws, Broken::NeverBack),
        (ready && draws, Broken::BackAfterReady),
        (alone != (f.held || f.gone || f.takers > 0), Broken::StoppedWrong),
    ]
    .into_iter()
    .filter_map(|(bad, b)| bad.then_some(b))
    .collect()
}

/// The rule a state with no step left breaks.
pub fn end(w: &World) -> Option<Broken> {
    (!w.done()).then_some(Broken::Stuck)
}
