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

//! give_back.rs `give_back`, one call: leave.rs `retire`, the function
//! stopped while the claim still names it, its grants torn down as released
//! ones, then release.rs `release`: stopped again, one release settled, and
//! when it was the last the entry goes and its wipe is owed in that same
//! hold (gone.rs); the release then steps the wipes itself.

use super::model_settle::{named, settle};
use super::model_stage::stage_rule::retired;
use super::model_stepper::Stepper;
use super::phase_rule::MS;
use super::world::World;
use super::world_thread::{Path, Thread};

pub(super) fn give(w: &mut World, p: usize, t: &mut Thread) {
    t.step += 1;
    match t.step - 1 {
        0 if w.procs[p].dead => t.path = Path::Done,
        0 => retire(w, p, t),
        1 | 3 if named(w, p, t.epoch) => w.stop_bme(),
        3 => t.path = Path::Done,
        2 => {
            let mine = w.runs.iter().position(|r| r.live && r.pid == p && r.epoch == t.epoch);
            if let Some(id) = mine {
                w.teardown(id, true);
                t.step = 2;
                t.at = w.now + if w.chance(8) { MS * w.draw(41) } else { 0 };
            }
        }
        4 if settle(w, p, t.epoch) => {}
        4 => t.path = Path::Done,
        5 => t.stepper = Some(Stepper::new(true)),
        6 => t.path = Path::Done,
        _ => {}
    }
}

fn retire(w: &mut World, p: usize, t: &mut Thread) {
    let Some(e) = w.entry.as_mut().filter(|e| e.pid == p) else {
        t.path = Path::Done;
        return;
    };
    match retired(e.stage) {
        Ok(stage) => (e.stage, e.leavers, t.epoch) = (stage, e.leavers + 1, e.epoch),
        Err(_) => t.path = Path::Done,
    }
}
