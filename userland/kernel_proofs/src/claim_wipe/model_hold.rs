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

//! A tenancy, in its driver's own code. Its driver resets its queue before
//! Bus Master goes on (a driver's duty, plan section 7), its state goes into
//! the function, it maps runs, may unmap one, may put the function in D3hot
//! (L64), then gives it back or exits; a sibling thread may give it back
//! too, and a kill may land at any point.

use super::bits_rule::{pmcsr_to, BME, D3HOT, INTX_DISABLE, MEM, NO_SNOOP_EN};
use super::model_stage::types::Stage;
use super::phase_rule::MS;
use super::world::World;
use super::world_thread::{Path, Thread};

pub(super) fn hold(w: &mut World, p: usize, t: &mut Thread) {
    if w.procs[p].dead {
        t.path = Path::Done;
        return;
    }
    t.at = w.now + MS * w.draw(41);
    // A driver's calls on a claim another thread gave back are refused.
    let ready = (p, t.epoch, Stage::Ready);
    if t.step < 2 && !w.entry.is_some_and(|e| (e.pid, e.epoch, e.stage) == ready) {
        t.step = 2;
    }
    match t.step {
        0 => {
            w.dev.queue.clear();
            let msi = if w.chance(2) { INTX_DISABLE } else { 0 };
            w.update_command(false, |v| v | MEM | BME | msi);
            if w.dev.caps.pcie && w.chance(2) {
                let d = w.devctl();
                w.set_devctl(false, d | NO_SNOOP_EN);
            }
            (w.dev.secret, w.dev.serving, t.step) = (Some(w.procs[p].svc), true, 1);
        }
        1 if w.procs[p].maps > 0 => {
            w.procs[p].maps -= 1;
            w.map_run(p, t.epoch);
        }
        1 => {
            let last = w.runs.iter().rposition(|r| r.live && r.pid == p);
            if let Some(id) = last.filter(|_| w.chance(4)) {
                w.dev.queue.retain(|&q| q != id);
                w.teardown(id, false);
            }
            if w.dev.caps.pm && w.chance(25) {
                let pm = w.pmcsr();
                w.set_pmcsr(false, pmcsr_to(pm, D3HOT));
            }
            t.step = 2;
        }
        _ if w.chance(4) => t.path = Path::Done,
        _ => (t.path, t.step) = (Path::Give, 0),
    }
}
