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

//! exit.rs, run once no thread of the group is left in the kernel: a claim
//! being made is orphaned for its own call to take back, any other counted
//! among its releases and its function stopped; every grant drained as a
//! released one; then exit_end settles the claim, which may owe its wipe,
//! and steps the wipes.

use super::model_settle::settle;
use super::model_stage::stage_rule::exited;
use super::model_stepper::Stepper;
use super::world::World;
use super::world_cases::KILLED;
use super::world_thread::{Path, Thread};

pub(super) fn exit(w: &mut World, p: usize, t: &mut Thread) {
    t.step += 1;
    match t.step - 1 {
        0 => begin_exit(w, p),
        1 if w.procs[p].held.is_some() => w.stop_bme(),
        2 => {
            if let Some(id) = w.runs.iter().position(|r| r.live && r.pid == p) {
                w.teardown(id, true);
                t.step = 2;
            }
        }
        3 => {
            if let Some(epoch) = w.procs[p].held {
                settle(w, p, epoch);
            }
        }
        4 => t.stepper = Some(Stepper::new(true)),
        5 => t.path = Path::Done,
        _ => {}
    }
}

fn begin_exit(w: &mut World, p: usize) {
    // A kill between attempts or in libc's wait: a claimant killed.
    w.seen[KILLED] += matches!(w.threads[3 * p].path, Path::Idle | Path::Claim) as u32;
    w.procs[p].dead = true;
    let Some(e) = w.entry.as_mut().filter(|e| e.pid == p) else { return };
    let (stage, counted) = exited(e.stage);
    e.stage = stage;
    if counted {
        e.leavers += 1;
        w.procs[p].held = Some(e.epoch);
    }
}
