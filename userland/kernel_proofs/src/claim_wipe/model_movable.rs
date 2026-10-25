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

//! Who may move now: a thread not waiting for its time, for the table
//! another holds, or (an exit) for its group's threads to leave the kernel;
//! when the next one is due; and when a seed is done.

use super::due_rule::pending;
use super::model_stepper::blocked;
use super::phase_rule::MS;
use super::world::World;
use super::world_thread::{Path, Thread};

pub(super) const REGISTRY: usize = 11;

fn active(w: &World, a: usize) -> bool {
    a >= 9 || !matches!(w.threads[a].path, Path::Off | Path::Done)
}

/// Inside a call into the kernel, which runs to its end once begun.
fn in_kernel(t: &Thread) -> bool {
    t.stepper.is_some() || (matches!(t.path, Path::Claim | Path::Give) && t.step > 0)
}

pub(super) fn movable(w: &World, a: usize) -> bool {
    if a == REGISTRY {
        return w.runs.iter().any(|r| r.registry && r.orphan_until <= w.now);
    }
    let t = &w.threads[a];
    let waits = match &t.stepper {
        Some(s) => blocked(w, s, a),
        None => takes_wipes(t) && w.wipes_by.is_some_and(|b| b != a),
    };
    // devices.rs: the exit begins once no thread of its group runs (L77).
    let busy = |b: usize| in_kernel(&w.threads[b]);
    let exits = a < 9 && a % 3 == 2 && t.step == 0 && (busy(a - 2) || busy(a - 1));
    active(w, a) && t.at <= w.now && !waits && !exits
}

/// The moves that take `WIPES` inside a claims hold: enter's and admit's
/// questions, and the owe in a release's or an exit's settle.
fn takes_wipes(t: &Thread) -> bool {
    matches!((t.path, t.step), (Path::Claim, 2 | 8) | (Path::Give, 4) | (Path::Exit, 3))
}

pub(super) fn next(w: &World) -> u64 {
    let threads = (0..11).filter(|&a| active(w, a)).map(|a| w.threads[a].at);
    let runs = w.runs.iter().filter(|r| r.registry).map(|r| r.orphan_until);
    threads.chain(runs).filter(|&t| t > w.now).min().unwrap_or(w.now + MS)
}

pub(super) fn quiet(w: &World) -> bool {
    let calls = w.threads[9..].iter().all(|t| t.stepper.is_none());
    let idle = (0..9).all(|a| !active(w, a)) && calls;
    idle && !w.rec.is_some_and(|r| pending(r.phase)) && !w.runs.iter().any(|r| r.registry)
}
